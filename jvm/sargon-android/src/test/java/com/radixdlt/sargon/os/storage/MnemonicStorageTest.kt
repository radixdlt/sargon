package com.radixdlt.sargon.os.storage

import android.security.keystore.KeyInfo
import android.util.Base64 as AndroidBase64
import androidx.biometric.BiometricPrompt
import androidx.datastore.core.DataStore
import androidx.datastore.preferences.core.Preferences
import androidx.datastore.preferences.core.PreferenceDataStoreFactory
import androidx.datastore.preferences.core.edit
import androidx.datastore.preferences.core.stringPreferencesKey
import com.radixdlt.sargon.os.driver.BiometricAuthorizationDriver
import com.radixdlt.sargon.os.driver.BiometricsFailure
import io.mockk.every
import io.mockk.mockk
import io.mockk.mockkConstructor
import io.mockk.mockkStatic
import io.mockk.unmockkConstructor
import io.mockk.unmockkStatic
import kotlinx.coroutines.CancellationException
import kotlinx.coroutines.test.TestScope
import kotlinx.coroutines.test.runTest
import kotlinx.coroutines.flow.first
import kotlinx.coroutines.flow.flow
import kotlinx.coroutines.flow.onStart
import org.junit.jupiter.api.Assertions.*
import org.junit.jupiter.api.Test
import org.junit.jupiter.params.ParameterizedTest
import org.junit.jupiter.params.provider.ValueSource
import org.junit.jupiter.api.io.TempDir
import java.io.File
import java.io.IOException
import java.util.Base64
import javax.crypto.Cipher
import javax.crypto.spec.SecretKeySpec

internal class MnemonicStorageTest {
    @TempDir lateinit var directory: File
    private val legacy = MnemonicStorage.LEGACY_ALIAS
    private val protected = MnemonicStorage.PROTECTED_ALIAS
    private val prefix = MnemonicStorage.PREFIX
    private fun secured(plaintext: String) = prefix + "new:" + plaintext

    private fun TestScope.store() = PreferenceDataStoreFactory.create(
        scope = backgroundScope
    ) { File(directory, "seeds.preferences_pb") }

    private suspend fun DataStore<Preferences>.put(name: String, value: String) = edit { it[stringPreferencesKey(name)] = value }
    private suspend fun DataStore<Preferences>.raw(name: String) = data.first()[stringPreferencesKey(name)]

    private class FakeCrypto : MnemonicStorageCrypto {
        val states = mutableMapOf(MnemonicStorage.LEGACY_ALIAS to MnemonicKeyState.Unprotected)
        var encryptFailure: ((String) -> Unit)? = null
        var deleteFailure = false
        var lookupFailureAlias: String? = null
        override fun keyState(alias: String): MnemonicKeyState {
            check(alias != lookupFailureAlias) { "Keystore unavailable" }
            return states[alias] ?: MnemonicKeyState.Missing
        }
        override fun ensureKeyExists(alias: String, mayCreate: Boolean) {
            if (keyState(alias) != MnemonicKeyState.Missing) return
            check(mayCreate)
            states[alias] = MnemonicKeyState.Protected
        }
        override fun delete(alias: String) { check(!deleteFailure); states.remove(alias) }
        override suspend fun encrypt(plaintext: String, alias: String): String {
            encryptFailure?.invoke(plaintext)
            check(keyState(alias) == MnemonicKeyState.Protected)
            return "new:$plaintext"
        }
        override suspend fun decrypt(ciphertext: String, alias: String): String {
            return when (keyState(alias)) {
                MnemonicKeyState.Missing -> error("Mnemonic encryption key is missing")
                MnemonicKeyState.Unprotected -> {
                    check(ciphertext.startsWith("old:"))
                    ciphertext.removePrefix("old:")
                }
                MnemonicKeyState.Protected -> {
                    check(ciphertext.startsWith("new:"))
                    ciphertext.removePrefix("new:")
                }
            }
        }
    }

    @ParameterizedTest
    @ValueSource(booleans = [false, true])
    fun unaffectedOrMissingLegacyKeyDoesNotReadRecords(legacyExists: Boolean) = runTest {
        val crypto = FakeCrypto().apply {
            states.clear()
            if (legacyExists) states[legacy] = MnemonicKeyState.Protected
        }
        val data = object : DataStore<Preferences> {
            override val data = flow<Preferences> { throw IOException("Preferences must not be read") }
            override suspend fun updateData(transform: suspend (Preferences) -> Preferences): Preferences =
                error("Unaffected storage must not be changed")
        }
        val retainedKeys = crypto.states.toMap()
        val storage = MnemonicStorage(data, crypto)

        assertEquals(0, storage.pendingMigrationCount().getOrThrow())
        assertEquals(MnemonicStorage.MigrationResult(0, 0, false), storage.migrate().getOrThrow())
        assertEquals(retainedKeys, crypto.states)
    }

    @Test fun corruptRecordDoesNotBlockSecuringOtherRecords() = runTest {
        val data = store()
        data.put("mnemonicA", "corrupt")
        data.put("mnemonicB", "old:seed B with passphrase")
        data.put("profile", "unrelated profile ciphertext")
        val crypto = FakeCrypto()
        val storage = MnemonicStorage(data, crypto)
        val report = storage.migrate().getOrThrow()
        assertEquals(1, report.migrated)
        assertEquals(1, report.remaining)
        assertEquals("corrupt", data.raw("mnemonicA"))
        assertEquals(secured("seed B with passphrase"), data.raw("mnemonicB"))
        assertEquals("seed B with passphrase", storage.read("mnemonicB").getOrThrow())
        assertTrue(crypto.states.containsKey(legacy))
        assertEquals("unrelated profile ciphertext", data.raw("profile"))
    }

    @ParameterizedTest
    @ValueSource(booleans = [true, false])
    fun cancellationRetainsProgressAndStopsBeforeTheNextRecord(callerCancelled: Boolean) = runTest {
        val data = store()
        for (name in listOf("A", "B", "C")) data.put("mnemonic$name", "old:seed $name")
        val crypto = FakeCrypto().apply { encryptFailure = {
            if (it == "seed B") {
                if (callerCancelled) throw CancellationException()
                throw BiometricsFailure(BiometricPrompt.ERROR_USER_CANCELED, "Cancelled")
            }
        } }
        val storage = MnemonicStorage(data, crypto)
        if (callerCancelled) {
            try { storage.migrate(); fail("Must propagate caller cancellation") }
            catch (_: CancellationException) { }
        } else {
            val report = storage.migrate().getOrThrow()
            assertEquals(1, report.migrated)
            assertEquals(2, report.remaining)
        }
        assertEquals(secured("seed A"), data.raw("mnemonicA"))
        assertEquals("old:seed B", data.raw("mnemonicB"))
        assertEquals("old:seed C", data.raw("mnemonicC"), "Cancellation must not prompt for the next mnemonic")
        assertTrue(crypto.states.containsKey(legacy))
    }

    @Test fun encryptionFailureDoesNotReplaceOriginalCiphertext() = runTest {
        val data = store()
        data.put("mnemonicA", "old:seed A")
        val crypto = FakeCrypto().apply { encryptFailure = { throw IOException("Encryption failed") } }
        assertEquals(1, MnemonicStorage(data, crypto).migrate().getOrThrow().remaining)
        assertEquals("old:seed A", data.raw("mnemonicA"))
        assertTrue(crypto.states.containsKey(legacy))
    }

    @Test fun cleanupFailureLeavesMigratedDataUsableAndCanBeRetried() = runTest {
        val data = store()
        data.put("mnemonicA", "old:seed A")
        val crypto = FakeCrypto().apply { deleteFailure = true }
        val storage = MnemonicStorage(data, crypto)
        val first = storage.migrate().getOrThrow()
        assertEquals(1, first.migrated)
        assertEquals(0, first.remaining)
        assertTrue(first.cleanupPending)
        assertEquals("seed A", storage.read("mnemonicA").getOrThrow())
        crypto.deleteFailure = false
        crypto.lookupFailureAlias = protected
        val retry = storage.migrate().getOrThrow()
        assertEquals(0, retry.migrated)
        assertEquals(0, retry.remaining)
        assertFalse(retry.cleanupPending)
        assertFalse(crypto.states.containsKey(legacy))
        crypto.lookupFailureAlias = null
        assertEquals("seed A", storage.read("mnemonicA").getOrThrow())
    }

    @Test fun failedCommitPreservesProgressAndRetrySkipsCompletedRecords() = runTest {
        val data = store()
        data.put("mnemonicA", "old:seed A")
        data.put("mnemonicB", "old:seed B")
        data.put("mnemonicC", secured("seed C"))
        var failCommit = true
        var reads = 0
        val underlying = data
        val failing = object : DataStore<Preferences> {
            override val data = underlying.data.onStart { reads++ }
            override suspend fun updateData(transform: suspend (Preferences) -> Preferences): Preferences {
                return underlying.updateData { current ->
                    val updated = transform(current)
                    val b = stringPreferencesKey("mnemonicB")
                    if (failCommit && updated[b] != current[b]) throw IOException("Disk unavailable")
                    updated
                }
            }
        }
        val crypto = FakeCrypto().apply { states[protected] = MnemonicKeyState.Protected }
        val report = MnemonicStorage(failing, crypto).migrate().getOrThrow()
        assertEquals(1, reads, "Migration must use the original snapshot without reloading after commits")
        assertEquals(1, report.migrated)
        assertEquals(1, report.remaining)
        assertEquals(secured("seed A"), data.raw("mnemonicA"))
        assertEquals("old:seed B", data.raw("mnemonicB"))
        assertEquals(secured("seed C"), data.raw("mnemonicC"))
        assertTrue(crypto.states.containsKey(legacy))
        failCommit = false
        val committedA = data.raw("mnemonicA")
        val resumed = MnemonicStorage(failing, crypto) // New coordinator models resuming after restart.
        val retry = resumed.migrate().getOrThrow()
        assertEquals(2, reads, "Each migration attempt must read one snapshot")
        assertEquals(1, retry.migrated)
        assertEquals(0, retry.remaining)
        assertEquals(committedA, data.raw("mnemonicA"), "Retry must retain the completed record")
        assertEquals(secured("seed C"), data.raw("mnemonicC"))
        assertFalse(crypto.states.containsKey(legacy))
        assertEquals("seed A", resumed.read("mnemonicA").getOrThrow())
        assertEquals("seed B", resumed.read("mnemonicB").getOrThrow())
        assertEquals(0, resumed.migrate().getOrThrow().migrated)
    }

    @ParameterizedTest
    @ValueSource(booleans = [false, true])
    fun replacementKeyFailurePreventsWritesAndPreservesRecords(lookupFails: Boolean) = runTest {
        val data = store()
        data.put("mnemonicA", secured("seed A"))
        val crypto = FakeCrypto().apply {
            if (lookupFails) lookupFailureAlias = protected
        }
        val storage = MnemonicStorage(data, crypto)
        val retained = data.data.first()
        val retainedKeys = crypto.states.toMap()
        assertTrue(storage.write("mnemonicA", "seed A").isFailure)
        assertEquals(retainedKeys, crypto.states, "Failed lookup/write must not delete or regenerate a key")
        assertEquals(retained, data.data.first())
    }

    @ParameterizedTest
    @ValueSource(booleans = [false, true])
    fun writesUseProtectedStorageAndLeaveLegacyRecordsUntouched(legacyProtected: Boolean) = runTest {
        val data = store()
        val original = if (legacyProtected) "new:seed A" else "old:seed A"
        data.put("mnemonicA", original)
        val crypto = FakeCrypto().apply {
            states[legacy] = if (legacyProtected) MnemonicKeyState.Protected else MnemonicKeyState.Unprotected
        }
        val storage = MnemonicStorage(data, crypto)
        storage.write("mnemonicB", "seed B").getOrThrow()
        assertEquals(secured("seed B"), data.raw("mnemonicB"))
        assertEquals(original, data.raw("mnemonicA"))
        assertEquals("seed A", storage.read("mnemonicA").getOrThrow())
        assertEquals("seed B", storage.read("mnemonicB").getOrThrow())
    }

    @Test fun writesReuseProtectedKeyWithoutReadingRecordsOrLegacyKey() = runTest {
        val data = store()
        val original = secured("seed A")
        data.put("mnemonicA", original)
        val crypto = FakeCrypto().apply {
            states[protected] = MnemonicKeyState.Protected
            lookupFailureAlias = legacy
        }
        val retainedKeys = crypto.states.toMap()
        var reads = 0
        val underlying = data
        val tracked = object : DataStore<Preferences> {
            override val data = underlying.data.onStart { reads++ }
            override suspend fun updateData(transform: suspend (Preferences) -> Preferences): Preferences =
                underlying.updateData(transform)
        }

        MnemonicStorage(tracked, crypto).write("mnemonicB", "seed B").getOrThrow()

        assertEquals(0, reads, "Reusing an existing key must not load mnemonic records")
        assertEquals(original, data.raw("mnemonicA"))
        assertEquals(secured("seed B"), data.raw("mnemonicB"))
        assertEquals(retainedKeys, crypto.states)
    }

    @Test fun firstWriteCreatesKeyButKeyLossPreservesRetainedRecords() = runTest {
        val data = store()
        data.put("profile", "unrelated profile ciphertext")
        val crypto = FakeCrypto().apply { states.clear() }
        val storage = MnemonicStorage(data, crypto)
        storage.write("mnemonicA", "seed A").getOrThrow()
        assertEquals(mapOf(protected to MnemonicKeyState.Protected), crypto.states)
        assertEquals(secured("seed A"), data.raw("mnemonicA"))
        assertEquals("seed A", storage.read("mnemonicA").getOrThrow())
        crypto.delete(protected)
        val retained = data.data.first()

        assertTrue(storage.write("mnemonicB", "seed B").isFailure)

        assertTrue(crypto.states.isEmpty(), "Retained ciphertext must prevent key regeneration")
        assertEquals(retained, data.data.first())
        assertEquals("unrelated profile ciphertext", data.raw("profile"))
    }

    @ParameterizedTest
    @ValueSource(booleans = [false, true])
    fun readsUseAuthorizedCipherOrLoadedTimedKey(timeWindow: Boolean) = runTest {
        val data = store()
        val secretKey = SecretKeySpec(ByteArray(32) { 7 }, EncryptionHelper.AES_ALGORITHM)
        val encrypted = EncryptionHelper.encrypt("seed A".toByteArray(), secretKey).getOrThrow()
        val stored = prefix + Base64.getEncoder().encodeToString(encrypted)
        data.put("mnemonicA", stored)
        val info = mockk<KeyInfo>()
        every { info.isUserAuthenticationRequired } returns true
        every { info.userAuthenticationValidityDurationSeconds } returns if (timeWindow) 30 else 0
        var authorized = false
        mockkStatic(AndroidBase64::class)
        mockkConstructor(KeySpec.Mnemonic::class)
        try {
            every { AndroidBase64.decode(any<String>(), AndroidBase64.DEFAULT) } answers {
                Base64.getMimeDecoder().decode(firstArg<String>())
            }
            every { anyConstructed<KeySpec.Mnemonic>().getSecretKey() } answers {
                assertFalse(authorized, "The key must not be looked up again after authorization")
                // Per-use access must consume the callback's cipher, even if the loaded key differs.
                Result.success(if (timeWindow) secretKey else SecretKeySpec(ByteArray(32) { 9 }, EncryptionHelper.AES_ALGORITHM))
            }
            every { anyConstructed<KeySpec.Mnemonic>().getKeyInfo(any()) } returns info
            val authorization = object : BiometricAuthorizationDriver {
                override val hasStrongAuthenticator = false
                override suspend fun authorize(cipher: Cipher?): Result<Cipher?> {
                    assertEquals(timeWindow, cipher == null)
                    authorized = true
                    return Result.success(if (timeWindow) null else {
                        EncryptionHelper.initDecryptCipher(encrypted, secretKey).getOrThrow()
                    })
                }
            }
            val storage = MnemonicStorage(data, AndroidMnemonicStorageCrypto(authorization))

            assertEquals("seed A", storage.read("mnemonicA").getOrThrow())
            assertTrue(authorized)
            assertEquals(stored, data.raw("mnemonicA"), "Reading must preserve the stored ciphertext")
        } finally {
            unmockkConstructor(KeySpec.Mnemonic::class)
            unmockkStatic(AndroidBase64::class)
        }
    }
}
