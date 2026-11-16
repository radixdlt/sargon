package com.radixdlt.sargon.os.driver

import android.content.Context
import androidx.biometric.BiometricPrompt
import androidx.datastore.core.DataStore
import androidx.datastore.preferences.core.PreferenceDataStoreFactory
import androidx.datastore.preferences.core.Preferences
import androidx.datastore.preferences.core.stringPreferencesKey
import androidx.test.core.app.ApplicationProvider
import androidx.test.ext.junit.runners.AndroidJUnit4
import androidx.test.filters.MediumTest
import com.radixdlt.sargon.CommonException
import com.radixdlt.sargon.Exactly32Bytes
import com.radixdlt.sargon.FactorSourceIdFromHash
import com.radixdlt.sargon.FactorSourceKind
import com.radixdlt.sargon.MnemonicWithPassphrase
import com.radixdlt.sargon.SecureStorageAccessErrorKind
import com.radixdlt.sargon.SecureStorageKey
import com.radixdlt.sargon.extensions.hex
import com.radixdlt.sargon.mnemonicWithPassphraseToJsonBytes
import com.radixdlt.sargon.newMnemonicWithPassphraseFromJsonBytes
import com.radixdlt.sargon.os.storage.MnemonicStorage
import com.radixdlt.sargon.os.storage.MnemonicKeyState
import com.radixdlt.sargon.os.storage.MnemonicStorageCrypto
import com.radixdlt.sargon.os.storage.decrypt
import com.radixdlt.sargon.os.storage.encrypt
import com.radixdlt.sargon.samples.sample
import com.radixdlt.sargon.secureStorageKeyIdentifier
import kotlinx.coroutines.flow.first
import kotlinx.coroutines.test.TestScope
import kotlinx.coroutines.test.runTest
import org.junit.After
import org.junit.Assert.assertEquals
import org.junit.Assert.assertFalse
import org.junit.Assert.assertNull
import org.junit.Assert.assertTrue
import org.junit.Test
import org.junit.runner.RunWith
import java.io.File
import java.util.UUID
import javax.crypto.KeyGenerator
import javax.crypto.SecretKey

/** Driver/mapping/DataStore integration; platform authentication is covered by the Keystore tests. */
@RunWith(AndroidJUnit4::class)
@MediumTest
class AndroidStorageDriverDeviceFactorSourceMnemonicTest {
    private val testContext: Context = ApplicationProvider.getApplicationContext()
    private val directory = File(testContext.cacheDir, "mnemonic-driver-${UUID.randomUUID()}").apply { mkdirs() }
    private val crypto = TestCrypto()
    private val key = SecureStorageKey.DeviceFactorSourceMnemonic(
        FactorSourceIdFromHash(FactorSourceKind.DEVICE, Exactly32Bytes.sample())
    )
    private val otherKey = SecureStorageKey.DeviceFactorSourceMnemonic(
        FactorSourceIdFromHash(FactorSourceKind.DEVICE, Exactly32Bytes.sample.other())
    )
    private val mnemonic = MnemonicWithPassphrase.sample()
    private val otherMnemonic = MnemonicWithPassphrase.sample.other().copy(passphrase = " \u00e9\u5bc6\u78bc test passphrase ")
    private val record get() = stringPreferencesKey("mnemonic${key.factorSourceId.body.hex}")
    private val otherRecord get() = stringPreferencesKey("mnemonic${otherKey.factorSourceId.body.hex}")

    @After
    fun deleteDatastore() {
        directory.deleteRecursively()
    }

    @Test
    fun mnemonicsRemainSeparateAcrossDriversAndScopedDeletion() = runTest {
        val preferences = preferences()
        val driver = newDriver(preferences)
        driver.saveData(key, mnemonicWithPassphraseToJsonBytes(mnemonic))
        driver.saveData(otherKey, mnemonicWithPassphraseToJsonBytes(otherMnemonic))
        val stored = preferences.data.first()
        assertEquals(setOf(record, otherRecord), preferences.mnemonicRecords().keys)
        val recreated = newDriver(preferences)
        assertEquals(mnemonic, newMnemonicWithPassphraseFromJsonBytes(requireNotNull(recreated.loadData(key))))
        assertEquals(otherMnemonic, newMnemonicWithPassphraseFromJsonBytes(requireNotNull(recreated.loadData(otherKey))))
        assertEquals("Reads must not rewrite ciphertext", stored, preferences.data.first())

        crypto.rejectAccess = true
        recreated.deleteDataForKey(key)
        assertEquals(stored.asMap() - record, preferences.data.first().asMap())
        assertNull(recreated.loadData(key))
        assertFalse(recreated.containsDataForKey(key))
        assertTrue(recreated.containsDataForKey(otherKey))
        crypto.rejectAccess = false
        assertEquals(otherMnemonic, newMnemonicWithPassphraseFromJsonBytes(requireNotNull(recreated.loadData(otherKey))))
    }

    @Test
    fun cancelledAccessDoesNotCreateOrReplaceMnemonicRecords() = runTest {
        val preferences = preferences()
        val driver = newDriver(preferences)
        crypto.rejectAccess = true
        assertUserCancelled(key, runCatching { driver.saveData(key, mnemonicWithPassphraseToJsonBytes(mnemonic)) }.exceptionOrNull())
        assertTrue(preferences.mnemonicRecords().isEmpty())
        crypto.rejectAccess = false
        driver.saveData(key, mnemonicWithPassphraseToJsonBytes(mnemonic))
        driver.saveData(otherKey, mnemonicWithPassphraseToJsonBytes(otherMnemonic))
        val stored = preferences.mnemonicRecords()
        crypto.rejectAccess = true

        assertUserCancelled(key, runCatching { driver.loadData(key) }.exceptionOrNull())
        assertUserCancelled(key, runCatching { driver.saveData(key, mnemonicWithPassphraseToJsonBytes(otherMnemonic)) }.exceptionOrNull())
        assertEquals("Cancelled reads/replacements must retain both ciphertexts", stored, preferences.mnemonicRecords())
        crypto.rejectAccess = false
        val recreated = newDriver(preferences)
        assertEquals(mnemonic, newMnemonicWithPassphraseFromJsonBytes(requireNotNull(recreated.loadData(key))))
        assertEquals(otherMnemonic, newMnemonicWithPassphraseFromJsonBytes(requireNotNull(recreated.loadData(otherKey))))
    }

    @Test
    fun mnemonicExistenceDoesNotDependOnKeystoreAvailability() = runTest {
        val preferences = preferences()
        val driver = newDriver(preferences)
        crypto.lookupFailure = IllegalStateException("Keystore unavailable")
        assertFalse("An absent record must not need Keystore access", driver.containsDataForKey(key))
        assertNull(driver.loadData(key))
        crypto.lookupFailure = null
        driver.saveData(key, mnemonicWithPassphraseToJsonBytes(mnemonic))
        driver.saveData(otherKey, mnemonicWithPassphraseToJsonBytes(otherMnemonic))
        val stored = preferences.data.first()
        crypto.lookupFailure = IllegalStateException("Keystore unavailable")
        assertTrue("An existing record must not need Keystore access", driver.containsDataForKey(key))
        crypto.lookupFailure = null
        crypto.delete(MnemonicStorage.PROTECTED_ALIAS)
        assertTrue("Retained ciphertext still exists when its key is lost", driver.containsDataForKey(key))
        assertTrue(runCatching { driver.loadData(key) }.exceptionOrNull() is CommonException.SecureStorageReadException)
        assertTrue(runCatching { driver.saveData(key, mnemonicWithPassphraseToJsonBytes(mnemonic)) }.exceptionOrNull()
            is CommonException.SecureStorageWriteException)
        assertEquals("Existence checks and failed reads must preserve records", stored, preferences.data.first())
    }

    private fun TestScope.preferences() = PreferenceDataStoreFactory.create(scope = backgroundScope) {
        File(directory, "encrypted.preferences_pb")
    }

    private suspend fun DataStore<Preferences>.mnemonicRecords() =
        data.first().asMap().filterKeys { it.name.startsWith("mnemonic") }

    private fun newDriver(preferences: DataStore<Preferences>) = AndroidStorageDriver(
        encryptedPreferencesDatastore = preferences,
        preferencesDatastore = preferences,
        deviceInfoDatastore = preferences,
        mnemonicStorage = MnemonicStorage(preferences, crypto)
    )

    private fun assertUserCancelled(key: SecureStorageKey, error: Throwable?) {
        assertTrue("Expected secure storage access failure, got $error", error is CommonException.SecureStorageAccessException)
        error as CommonException.SecureStorageAccessException
        assertEquals(SecureStorageAccessErrorKind.USER_CANCELLED, error.errorKind)
        assertEquals(secureStorageKeyIdentifier(key), error.key)
    }

    /** Software AES with access denial injected at the crypto boundary. */
    private class TestCrypto : MnemonicStorageCrypto {
        private val keys = mutableMapOf<String, SecretKey>()
        var rejectAccess = false
        var lookupFailure: Exception? = null

        override fun keyState(alias: String): MnemonicKeyState {
            lookupFailure?.let { throw it }
            return if (alias in keys) MnemonicKeyState.Protected else MnemonicKeyState.Missing
        }

        override fun ensureKeyExists(alias: String, mayCreate: Boolean) {
            if (alias !in keys) {
                check(mayCreate)
                keys[alias] = KeyGenerator.getInstance("AES").apply { init(256) }.generateKey()
            }
        }

        override fun delete(alias: String) { keys.remove(alias) }

        override suspend fun encrypt(plaintext: String, alias: String): String {
            checkAccess()
            return plaintext.encrypt(requireNotNull(keys[alias])).getOrThrow()
        }

        override suspend fun decrypt(ciphertext: String, alias: String): String {
            checkAccess()
            return ciphertext.decrypt(requireNotNull(keys[alias])).getOrThrow()
        }

        private fun checkAccess() {
            if (rejectAccess) throw BiometricsFailure(BiometricPrompt.ERROR_USER_CANCELED, "The user cancelled")
        }
    }
}
