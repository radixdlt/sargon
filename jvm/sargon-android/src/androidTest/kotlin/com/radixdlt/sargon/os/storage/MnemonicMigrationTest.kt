package com.radixdlt.sargon.os.storage

import android.content.Intent
import android.os.Build
import android.security.keystore.KeyGenParameterSpec
import android.security.keystore.KeyInfo
import android.security.keystore.KeyProperties
import android.util.Log
import androidx.datastore.preferences.core.PreferenceDataStoreFactory
import androidx.datastore.preferences.core.stringPreferencesKey
import androidx.test.core.app.ActivityScenario
import androidx.test.ext.junit.runners.AndroidJUnit4
import androidx.test.filters.SdkSuppress
import androidx.test.platform.app.InstrumentationRegistry
import com.radixdlt.sargon.os.driver.BiometricAuthorizationDriver
import com.radixdlt.sargon.os.driver.BiometricsHandler
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.SupervisorJob
import kotlinx.coroutines.cancelAndJoin
import kotlinx.coroutines.flow.first
import kotlinx.coroutines.runBlocking
import kotlinx.coroutines.delay
import org.junit.After
import org.junit.Assert.*
import org.junit.Assume.assumeTrue
import org.junit.Test
import org.junit.runner.RunWith
import java.io.File
import java.security.KeyStore
import java.util.UUID
import javax.crypto.Cipher
import javax.crypto.KeyGenerator
import javax.crypto.SecretKeyFactory

/** Opt-in: approve A's two prompts, cancel B's read, then approve mixed reads and resumed migration. */
@RunWith(AndroidJUnit4::class)
@SdkSuppress(minSdkVersion = Build.VERSION_CODES.O_MR1)
class MnemonicMigrationTest {
    private val context = InstrumentationRegistry.getInstrumentation().targetContext
    private val keyStore = KeyStore.getInstance("AndroidKeyStore").apply { load(null) }

    @After fun cleanup() {
        if (context.packageName == "com.radixdlt.sargon.test") {
            keyStore.deleteEntry(MnemonicStorage.LEGACY_ALIAS)
            keyStore.deleteEntry(MnemonicStorage.PROTECTED_ALIAS)
        }
    }

    @Test fun partialMigrationResumesWithRealKeystoreAndAuthentication() = runBlocking<Unit> {
        assumeTrue(InstrumentationRegistry.getArguments().getString("runManualKeystoreAuthTests") == "true")
        assertEquals("com.radixdlt.sargon.test", context.packageName)
        assertFalse(keyStore.containsAlias(MnemonicStorage.LEGACY_ALIAS))
        assertFalse(keyStore.containsAlias(MnemonicStorage.PROTECTED_ALIAS))
        val generator = KeyGenerator.getInstance("AES", "AndroidKeyStore")
        generator.init(KeyGenParameterSpec.Builder(MnemonicStorage.LEGACY_ALIAS,
            KeyProperties.PURPOSE_ENCRYPT or KeyProperties.PURPOSE_DECRYPT)
            .setBlockModes(KeyProperties.BLOCK_MODE_GCM)
            .setEncryptionPaddings(KeyProperties.ENCRYPTION_PADDING_NONE)
            .setRandomizedEncryptionRequired(false).setKeySize(256).build())
        val oldKey = generator.generateKey()
        val job = SupervisorJob()
        val file = File(context.cacheDir, "migration-${UUID.randomUUID()}.preferences_pb")
        val data = PreferenceDataStoreFactory.create(scope = CoroutineScope(job + Dispatchers.IO)) { file }
        val a = stringPreferencesKey("mnemonicA")
        val b = stringPreferencesKey("mnemonicB")
        val oldA = "disposable seed A and passphrase".encrypt(oldKey).getOrThrow()
        val oldB = "disposable seed B".encrypt(oldKey).getOrThrow()
        data.write(a, oldA).getOrThrow()
        data.write(b, oldB).getOrThrow()
        val handler = BiometricsHandler("Mnemonic migration test")
        val intent = Intent(context, ZeroTimeoutTestActivity::class.java).addFlags(Intent.FLAG_ACTIVITY_NEW_TASK)
        try {
            ActivityScenario.launch<ZeroTimeoutTestActivity>(intent).use { scenario ->
                scenario.onActivity { handler.register(it) }
                var authorizationRequests = 0
                val authorization = object : BiometricAuthorizationDriver {
                    override val hasStrongAuthenticator get() = handler.hasStrongAuthenticator
                    override suspend fun authorize(cipher: Cipher?): Result<Cipher?> {
                        authorizationRequests++
                        return handler.askForBiometrics(cipher)
                    }
                }
                var storage = MnemonicStorage(data, AndroidMnemonicStorageCrypto(authorization))
                val first = storage.migrate().getOrThrow()
                assertEquals(1, first.migrated)
                assertEquals(1, first.remaining)
                assertEquals("Two prompts for A, then cancellation of B's read", 3, authorizationRequests)
                assertTrue(data.data.first()[a]!!.startsWith(MnemonicStorage.PREFIX))
                assertEquals(oldB, data.data.first()[b])
                assertTrue(keyStore.containsAlias(MnemonicStorage.LEGACY_ALIAS))
                Log.i("MnemonicMigrationTest", "PARTIAL_COMMIT_OK")
                assertEquals("disposable seed A and passphrase", storage.read(a.name).getOrThrow())
                assertEquals("disposable seed B", storage.read(b.name).getOrThrow())
                storage = MnemonicStorage(data, AndroidMnemonicStorageCrypto(authorization))
                val second = storage.migrate().getOrThrow()
                assertEquals(1, second.migrated)
                assertEquals(0, second.remaining)
                assertFalse(second.cleanupPending)
                assertFalse(keyStore.containsAlias(MnemonicStorage.LEGACY_ALIAS))
                assertEquals("disposable seed A and passphrase", storage.read(a.name).getOrThrow())
                assertEquals("disposable seed B", storage.read(b.name).getOrThrow())
                assertEquals("Migration must not request a verification decrypt", 9, authorizationRequests)
                val storedA = data.data.first()[a]!!.removePrefix(MnemonicStorage.PREFIX)
                val spec = KeySpec.Mnemonic(MnemonicStorage.PROTECTED_ALIAS)
                val key = requireNotNull(spec.getSecretKey().getOrThrow())
                val info = SecretKeyFactory.getInstance(key.algorithm, "AndroidKeyStore")
                    .getKeySpec(key, KeyInfo::class.java) as KeyInfo
                assertTrue(info.isUserAuthenticationRequired)
                if (info.userAuthenticationValidityDurationSeconds > 0) {
                    assertEquals(30, info.userAuthenticationValidityDurationSeconds)
                    assertEquals("disposable seed A and passphrase", storedA.decrypt(spec).getOrThrow())
                    Log.i("MnemonicMigrationTest", "TIMED_AUTH_WINDOW_EXPIRY_WAIT")
                    delay(31_000)
                }
                assertTrue("A fresh cipher must not work without current authentication",
                    storedA.decrypt(spec).exceptionOrNull().isAuthenticationFailure())
                Log.i("MnemonicMigrationTest", "RESUME_AND_RETIREMENT_OK")

                val retained = data.data.first()
                keyStore.deleteEntry(MnemonicStorage.PROTECTED_ALIAS)
                assertTrue("Key loss must not hide a retained record", storage.contains(a.name))
                assertTrue(storage.read(a.name).isFailure)
                assertTrue(storage.write(a.name, "disposable seed A and passphrase").isFailure)
                assertFalse("A referenced key must never be regenerated", keyStore.containsAlias(MnemonicStorage.PROTECTED_ALIAS))
                assertEquals(retained, data.data.first())
                Log.i("MnemonicMigrationTest", "MISSING_KEY_PRESERVES_RECORDS_OK")
            }
        } finally {
            job.cancelAndJoin()
            file.delete()
        }
    }
}
