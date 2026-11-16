package com.radixdlt.sargon.os.storage

import android.app.KeyguardManager
import android.content.Context
import android.content.Intent
import android.os.Build
import android.os.Bundle
import android.security.keystore.KeyGenParameterSpec
import android.security.keystore.KeyInfo
import android.security.keystore.KeyProperties
import android.security.keystore.UserNotAuthenticatedException
import android.widget.TextView
import androidx.biometric.BiometricManager
import androidx.biometric.BiometricManager.Authenticators.BIOMETRIC_STRONG
import androidx.biometric.BiometricManager.Authenticators.DEVICE_CREDENTIAL
import androidx.biometric.BiometricPrompt
import androidx.core.content.ContextCompat
import androidx.datastore.preferences.core.PreferenceDataStoreFactory
import androidx.datastore.preferences.core.stringPreferencesKey
import androidx.fragment.app.FragmentActivity
import androidx.test.core.app.ActivityScenario
import androidx.test.ext.junit.runners.AndroidJUnit4
import androidx.test.filters.SdkSuppress
import androidx.test.platform.app.InstrumentationRegistry
import com.radixdlt.sargon.os.driver.BiometricsHandler
import com.radixdlt.sargon.os.driver.AndroidBiometricAuthorizationDriver
import com.radixdlt.sargon.os.driver.BiometricAuthorizationDriver
import kotlinx.coroutines.CompletableDeferred
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.SupervisorJob
import kotlinx.coroutines.cancelAndJoin
import kotlinx.coroutines.flow.first
import kotlinx.coroutines.runBlocking
import kotlinx.coroutines.withTimeout
import org.junit.After
import org.junit.Assert.assertEquals
import org.junit.Assert.assertFalse
import org.junit.Assert.assertTrue
import org.junit.Assume.assumeTrue
import org.junit.Before
import org.junit.Test
import org.junit.runner.RunWith
import java.io.File
import java.security.KeyStore
import java.util.UUID
import javax.crypto.Cipher
import javax.crypto.KeyGenerator
import javax.crypto.SecretKey
import javax.crypto.SecretKeyFactory

class ZeroTimeoutTestActivity : FragmentActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContentView(TextView(this).apply { text = "Disposable zero-timeout experiment" })
    }
}

/** Real Keystore and BiometricPrompt regression tests for the minimal authentication fix. */
@RunWith(AndroidJUnit4::class)
@SdkSuppress(minSdkVersion = Build.VERSION_CODES.R)
class ZeroTimeoutExperimentTest {
    private val context = InstrumentationRegistry.getInstrumentation().targetContext
    private val aliases = mutableListOf<String>()
    private val keyStore by lazy { KeyStore.getInstance("AndroidKeyStore").apply { load(null) } }

    @Before
    fun requireIsolatedSecureDevice() {
        assertEquals("Never run in the wallet's process", "com.radixdlt.sargon.test", context.packageName)
        assumeTrue("Requires a configured device PIN, pattern, or password",
            (context.getSystemService(Context.KEYGUARD_SERVICE) as KeyguardManager).isDeviceSecure)
    }

    @After
    fun cleanup() {
        aliases.forEach { keyStore.deleteEntry(it) }
    }

    @Test
    fun newKeyRequiresPerOperationAuthenticationAndRejectsUnauthenticatedUse() = runBlocking<Unit> {
        val spec = newSpec()
        val crypto = AndroidMnemonicStorageCrypto(AndroidBiometricAuthorizationDriver(BiometricsHandler("Key inspection")))
        assertEquals(MnemonicKeyState.Missing, crypto.keyState(spec.alias))
        var authorizationRequested = false
        val request = KeystoreAccessRequest.ForMnemonic(
            hasStrongAuthenticator = false,
            authorize = { authorizationRequested = true; Result.success(it) },
            mnemonicKeySpec = spec
        )
        assertTrue(request.requestAuthorization(KeystoreAccessRequest.Purpose.Encrypt).isFailure)
        assertTrue(request.requestAuthorization().isFailure)
        assertFalse("A missing key must fail before asking for authorization", authorizationRequested)
        assertFalse(keyStore.containsAlias(spec.alias))
        val key = spec.generateSecretKey().getOrThrow()
        key.info().assertPerOperationAuthentication()
        assertTrue("A fresh operation must require authentication", "dummy seed".encrypt(spec).exceptionOrNull().isAuthenticationFailure())
        assertEquals(MnemonicKeyState.Protected, crypto.keyState(spec.alias))
        assertTrue("Inspection must not authorize another operation", "dummy seed".encrypt(spec).exceptionOrNull().isAuthenticationFailure())
        keyStore.deleteEntry(spec.alias)
        assertEquals(MnemonicKeyState.Missing, crypto.keyState(spec.alias))
        assertFalse("Inspection must not regenerate a deleted key", keyStore.containsAlias(spec.alias))
    }

    @Test
    fun existingUnrestrictedKeySupportsAuthenticatedReadAndWrite() = verifyExistingKeyRoundtrip(null)

    @Test
    fun existingThirtySecondKeySupportsAuthenticatedReadAndWrite() = verifyExistingKeyRoundtrip(30)

    private fun verifyExistingKeyRoundtrip(timeoutSeconds: Int?) = runBlocking {
        requireManualAuthentication()
        val job = SupervisorJob()
        val file = File(context.cacheDir, "legacy-${UUID.randomUUID()}.preferences_pb")
        val storage = PreferenceDataStoreFactory.create(scope = CoroutineScope(job + Dispatchers.IO)) { file }
        val entry = stringPreferencesKey("mnemonic-dummy")
        val activity = CompletableDeferred<ZeroTimeoutTestActivity>()
        val intent = Intent(context, ZeroTimeoutTestActivity::class.java).addFlags(Intent.FLAG_ACTIVITY_NEW_TASK)
        try {
            ActivityScenario.launch<ZeroTimeoutTestActivity>(intent).use { scenario ->
                scenario.onActivity { activity.complete(it) }
                var authorizationCount = 0
                val authorization = object : BiometricAuthorizationDriver {
                    override val hasStrongAuthenticator = true
                    override suspend fun authorize(cipher: Cipher?): Result<Cipher?> {
                        assertEquals(timeoutSeconds != null && authorizationCount == 0, cipher == null)
                        authorizationCount++
                        return Result.success(authenticate(activity.await(), cipher))
                    }
                }
                val mnemonics = MnemonicStorage(storage, AndroidMnemonicStorageCrypto(authorization))
                val legacySpec = KeySpec.Mnemonic()
                aliases.add(legacySpec.alias)
                aliases.add(MnemonicStorage.PROTECTED_ALIAS)
                assertFalse(keyStore.containsAlias(legacySpec.alias))
                assertFalse(keyStore.containsAlias(MnemonicStorage.PROTECTED_ALIAS))
                // Reproduce the historical key configuration independently of the fixed key generator.
                val builder = KeyGenParameterSpec.Builder(legacySpec.alias,
                    KeyProperties.PURPOSE_ENCRYPT or KeyProperties.PURPOSE_DECRYPT)
                    .setBlockModes(KeyProperties.BLOCK_MODE_GCM)
                    .setEncryptionPaddings(KeyProperties.ENCRYPTION_PADDING_NONE)
                    .setRandomizedEncryptionRequired(false)
                    .setKeySize(256)
                if (timeoutSeconds != null) {
                    builder.setUserAuthenticationRequired(true)
                        .setInvalidatedByBiometricEnrollment(false)
                        .setUserAuthenticationParameters(timeoutSeconds,
                            KeyProperties.AUTH_BIOMETRIC_STRONG or KeyProperties.AUTH_DEVICE_CREDENTIAL)
                }
                val generator = KeyGenerator.getInstance("AES", "AndroidKeyStore")
                generator.init(builder.build())
                val oldKey = generator.generateKey()
                val oldInfo = oldKey.info()
                if (timeoutSeconds != null) authenticate(activity.await(), null)
                val oldCiphertext = "old disposable seed".encrypt(oldKey).getOrThrow()
                storage.write(entry, oldCiphertext).getOrThrow()

                // The fixed SDK must read the original bytes, not replace the key or rewrite the record.
                assertEquals("old disposable seed", mnemonics.read(entry.name).getOrThrow())
                assertEquals(oldCiphertext, storage.data.first()[entry])
                mnemonics.write(entry.name, "another disposable seed").getOrThrow()
                assertEquals("another disposable seed", mnemonics.read(entry.name).getOrThrow())
                assertTrue(storage.data.first()[entry]!!.startsWith(MnemonicStorage.PREFIX))
                assertEquals(3, authorizationCount)
                assertEquals("old disposable seed", oldCiphertext.decrypt(legacySpec).getOrThrow())
                val retainedInfo = requireNotNull(legacySpec.getSecretKey().getOrThrow()).info()
                assertEquals(oldInfo.isUserAuthenticationRequired, retainedInfo.isUserAuthenticationRequired)
                assertEquals(oldInfo.userAuthenticationValidityDurationSeconds,
                    retainedInfo.userAuthenticationValidityDurationSeconds)
            }
        } finally {
            job.cancelAndJoin()
            file.delete()
        }
    }

    /**
     * Run on a secure device with a PIN/password but no enrolled strong biometrics.
     * The existing per-operation key models the state left after biometrics are removed;
     * this test does not change the device's authentication settings.
     */
    @Test
    fun perOperationKeyRemainsReadableWhenOnlyDeviceCredentialIsAvailable() = runBlocking {
        requireManualAuthentication()
        val manager = BiometricManager.from(context)
        assumeTrue("Requires a configured device credential",
            manager.canAuthenticate(DEVICE_CREDENTIAL) == BiometricManager.BIOMETRIC_SUCCESS)
        assumeTrue("Requires no available strong biometrics",
            manager.canAuthenticate(BIOMETRIC_STRONG) != BiometricManager.BIOMETRIC_SUCCESS)

        val job = SupervisorJob()
        val file = File(context.cacheDir, "credential-only-${UUID.randomUUID()}.preferences_pb")
        val storage = PreferenceDataStoreFactory.create(scope = CoroutineScope(job + Dispatchers.IO)) { file }
        val entry = stringPreferencesKey("mnemonic-dummy")
        val plaintext = "disposable credential-only seed"
        val activity = CompletableDeferred<ZeroTimeoutTestActivity>()
        val handler = BiometricsHandler("PIN-only mnemonic read")
        val intent = Intent(context, ZeroTimeoutTestActivity::class.java).addFlags(Intent.FLAG_ACTIVITY_NEW_TASK)
        try {
            ActivityScenario.launch<ZeroTimeoutTestActivity>(intent).use { scenario ->
                scenario.onActivity {
                    handler.register(it)
                    activity.complete(it)
                }
                assertFalse(handler.hasStrongAuthenticator)

                // Seed the same key/data state that a former strong-biometric session creates.
                // Android 11+ also permits the current device credential to authorize this cipher.
                val originalSpec = KeySpec.Mnemonic()
                aliases.add(originalSpec.alias)
                aliases.add(MnemonicStorage.PROTECTED_ALIAS)
                assertFalse(keyStore.containsAlias(originalSpec.alias))
                assertFalse(keyStore.containsAlias(MnemonicStorage.PROTECTED_ALIAS))
                val originalKey = originalSpec
                    .withAuthenticationPolicy(hasStrongAuthenticator = true)
                    .generateSecretKey().getOrThrow()
                // Seed an unprefixed legacy record independently of the current v2-only writer.
                val (cipher, ivBytes) = EncryptionHelper.initEncryptCipher(originalKey).getOrThrow()
                val authorizedCipher = requireNotNull(authenticate(activity.await(), cipher))
                storage.write(entry, plaintext.encrypt(authorizedCipher, ivBytes).getOrThrow()).getOrThrow()
                val ciphertext = requireNotNull(storage.data.first()[entry])
                originalKey.info().assertPerOperationAuthentication()

                var credentialAuthenticationCount = 0
                val authorization = object : BiometricAuthorizationDriver {
                    override val hasStrongAuthenticator get() = handler.hasStrongAuthenticator
                    override suspend fun authorize(cipher: Cipher?) = withTimeout(90_000) {
                        handler.askForBiometrics(cipher).onSuccess { credentialAuthenticationCount++ }
                    }
                }
                val current = MnemonicStorage(storage, AndroidMnemonicStorageCrypto(authorization))
                assertEquals("Existing protected keys must not be migrated", 0, current.migrate().getOrThrow().migrated)
                assertEquals("Inspection must not request authentication", 0, credentialAuthenticationCount)
                assertTrue("Inspection must not grant decryption permission", ciphertext.decrypt(originalKey).exceptionOrNull().isAuthenticationFailure())
                assertEquals(ciphertext, storage.data.first()[entry])

                assertEquals(plaintext, current.read(entry.name).getOrThrow())
                assertEquals("The read must authenticate the original cipher", 1, credentialAuthenticationCount)
                assertEquals("Reading must not rewrite ciphertext", ciphertext, storage.data.first()[entry])
                requireNotNull(originalSpec.getSecretKey().getOrThrow()).info().assertPerOperationAuthentication()
                current.write(entry.name, "updated disposable seed").getOrThrow()
                assertEquals("updated disposable seed", current.read(entry.name).getOrThrow())
                assertTrue(storage.data.first()[entry]!!.startsWith(MnemonicStorage.PREFIX))
                assertEquals("Each read/write must request authorization", 3, credentialAuthenticationCount)
                assertTrue("Previous prompts must not authorize a fresh cipher",
                    ciphertext.decrypt(originalKey).exceptionOrNull().isAuthenticationFailure())
            }
        } finally {
            job.cancelAndJoin()
            file.delete()
        }
    }

    private fun requireManualAuthentication() {
        assumeTrue("Interactive test: set runManualKeystoreAuthTests=true to approve prompts manually",
            InstrumentationRegistry.getArguments().getString("runManualKeystoreAuthTests").toBoolean())
    }

    private suspend fun authenticate(activity: FragmentActivity, cipher: Cipher?): Cipher? {
        val result = CompletableDeferred<Cipher?>()
        activity.runOnUiThread {
            val prompt = BiometricPrompt(activity, ContextCompat.getMainExecutor(activity),
                object : BiometricPrompt.AuthenticationCallback() {
                    override fun onAuthenticationSucceeded(value: BiometricPrompt.AuthenticationResult) {
                        result.complete(if (cipher == null) null else requireNotNull(value.cryptoObject?.cipher))
                    }
                    override fun onAuthenticationError(code: Int, message: CharSequence) {
                        result.completeExceptionally(IllegalStateException("Authentication error $code: $message"))
                    }
                })
            val info = BiometricPrompt.PromptInfo.Builder()
                .setTitle("Disposable zero-timeout test")
                .setAllowedAuthenticators(BIOMETRIC_STRONG or DEVICE_CREDENTIAL)
                .build()
            if (cipher == null) prompt.authenticate(info)
            else prompt.authenticate(info, BiometricPrompt.CryptoObject(cipher))
        }
        return withTimeout(90_000) { result.await() }
    }

    private fun newSpec() = KeySpec.Mnemonic("zero-timeout-${UUID.randomUUID()}").also { aliases.add(it.alias) }
    private fun SecretKey.info() = SecretKeyFactory.getInstance(algorithm, "AndroidKeyStore")
        .getKeySpec(this, KeyInfo::class.java) as KeyInfo
    private fun KeyInfo.assertPerOperationAuthentication() {
        assertTrue(isUserAuthenticationRequired)
        // Android 11 reports -1; newer Keystore implementations report 0 for per-operation keys.
        assertTrue("Expected per-operation authentication, got timeout $userAuthenticationValidityDurationSeconds",
            userAuthenticationValidityDurationSeconds <= 0)
    }

}

internal fun Throwable?.isAuthenticationFailure() = generateSequence(this) { it.cause }
    .any { failure ->
        when {
            failure is UserNotAuthenticatedException -> true
            Build.VERSION.SDK_INT >= Build.VERSION_CODES.TIRAMISU ->
                failure is android.security.KeyStoreException && failure.requiresUserAuthentication()
            // Before API 33, the framework maps Keymaster's auth error to this message.
            else -> failure.javaClass.name == "android.security.KeyStoreException" &&
                failure.message?.contains("Key user not authenticated", ignoreCase = true) == true
        }
    }
