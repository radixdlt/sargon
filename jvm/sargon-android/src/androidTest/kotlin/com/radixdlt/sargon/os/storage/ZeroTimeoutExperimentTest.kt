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
import android.util.Log
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
    fun newKeyRequiresPerOperationAuthenticationAndRejectsUnauthenticatedUse() {
        val spec = newSpec()
        val key = spec.getOrGenerateSecretKey().getOrThrow()
        key.info().assertPerOperationAuthentication()
        val failure = "dummy seed".encrypt(spec).exceptionOrNull()
        assertTrue("Expected authentication failure, got $failure", failure.isAuthenticationFailure())
    }

    @Test
    fun existingUnrestrictedKeyAndCiphertextAreNotChangedByTheNewDefault() {
        val spec = newSpec()
        val generator = KeyGenerator.getInstance("AES", "AndroidKeyStore")
        generator.init(KeyGenParameterSpec.Builder(spec.alias, KeyProperties.PURPOSE_ENCRYPT or KeyProperties.PURPOSE_DECRYPT)
            .setBlockModes(KeyProperties.BLOCK_MODE_GCM)
            .setEncryptionPaddings(KeyProperties.ENCRYPTION_PADDING_NONE)
            .setRandomizedEncryptionRequired(false)
            .build())
        generator.generateKey()
        val ciphertext = "old dummy seed".encrypt(spec).getOrThrow()

        assertFalse(spec.getOrGenerateSecretKey().getOrThrow().info().isUserAuthenticationRequired)
        assertEquals("old dummy seed", ciphertext.decrypt(spec).getOrThrow())
    }

    @Test
    fun explicitTimedFallbackStillUsesThirtySeconds() {
        val spec = newSpec().apply { setAuthenticationTimeout() }
        val info = spec.getOrGenerateSecretKey().getOrThrow().info()
        assertTrue(info.isUserAuthenticationRequired)
        assertEquals(30, info.userAuthenticationValidityDurationSeconds)
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
                val request = KeystoreAccessRequest.ForMnemonic(hasStrongAuthenticator = true, authorize = { args ->
                    assertEquals(timeoutSeconds != null, args is KeystoreAccessRequest.AuthorizationArgs.TimeWindowAuth)
                    val cipher = authenticate(activity.await(), args.cipher)
                    Result.success(when (args) {
                        is KeystoreAccessRequest.AuthorizationArgs.Encrypt -> args.copy(cipher = requireNotNull(cipher))
                        is KeystoreAccessRequest.AuthorizationArgs.Decrypt -> args.copy(cipher = requireNotNull(cipher))
                        KeystoreAccessRequest.AuthorizationArgs.TimeWindowAuth -> args
                    })
                })
                aliases.add(request.keySpec.alias)
                assertFalse(keyStore.containsAlias(request.keySpec.alias))
                // Reproduce the historical key configuration independently of the fixed key generator.
                val builder = KeyGenParameterSpec.Builder(request.keySpec.alias,
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
                assertEquals("old disposable seed", storage.read(entry, request).getOrThrow())
                assertEquals(oldCiphertext, storage.data.first()[entry])
                storage.write(entry, "another disposable seed", request).getOrThrow()
                assertEquals("another disposable seed", storage.read(entry, request).getOrThrow())
                assertEquals("old disposable seed", oldCiphertext.decrypt(request.keySpec).getOrThrow())
                val retainedInfo = request.keySpec.getOrGenerateSecretKey().getOrThrow().info()
                assertEquals(oldInfo.isUserAuthenticationRequired, retainedInfo.isUserAuthenticationRequired)
                assertEquals(oldInfo.userAuthenticationValidityDurationSeconds,
                    retainedInfo.userAuthenticationValidityDurationSeconds)
            }
        } finally {
            job.cancelAndJoin()
            file.delete()
        }
    }

    @Test
    fun authenticatedWriteAndReadUsePromptAuthorizedCipher() = runBlocking {
        requireManualAuthentication()
        val job = SupervisorJob()
        val file = File(context.cacheDir, "zero-timeout-${UUID.randomUUID()}.preferences_pb")
        val storage = PreferenceDataStoreFactory.create(scope = CoroutineScope(job + Dispatchers.IO)) { file }
        val entry = stringPreferencesKey("mnemonic-dummy")
        val plaintext = "disposable dummy seed"
        val activity = CompletableDeferred<ZeroTimeoutTestActivity>()
        val intent = Intent(context, ZeroTimeoutTestActivity::class.java).addFlags(Intent.FLAG_ACTIVITY_NEW_TASK)
        try {
            ActivityScenario.launch<ZeroTimeoutTestActivity>(intent).use { scenario ->
                scenario.onActivity { activity.complete(it) }
                val request = KeystoreAccessRequest.ForMnemonic(hasStrongAuthenticator = true, authorize = { args ->
                    val cipher = requireNotNull(authenticate(activity.await(), requireNotNull(args.cipher)))
                    when (args) {
                        is KeystoreAccessRequest.AuthorizationArgs.Encrypt -> args.copy(cipher = cipher)
                        is KeystoreAccessRequest.AuthorizationArgs.Decrypt -> args.copy(cipher = cipher)
                        else -> error("Expected a per-operation CryptoObject")
                    }.let { Result.success(it) }
                })
                // The library test APK owns a separate UID/Keystore, never the wallet's alias.
                aliases.add(request.keySpec.alias)
                assertFalse(keyStore.containsAlias(request.keySpec.alias))
                storage.write(entry, plaintext, request).getOrThrow()
                val stored = storage.data.first()[entry]!!

                assertEquals(plaintext, storage.read(entry, request).getOrThrow())
                assertEquals(stored, storage.data.first()[entry])
                // A fresh cipher is not authorized by a previous prompt.
                assertTrue(stored.decrypt(request.keySpec).exceptionOrNull().isAuthenticationFailure())
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
                val originalRequest = KeystoreAccessRequest.ForMnemonic(hasStrongAuthenticator = true, authorize = { args ->
                    val cipher = requireNotNull(authenticate(activity.await(), requireNotNull(args.cipher)))
                    Result.success(when (args) {
                        is KeystoreAccessRequest.AuthorizationArgs.Encrypt -> args.copy(cipher = cipher)
                        is KeystoreAccessRequest.AuthorizationArgs.Decrypt -> args.copy(cipher = cipher)
                        else -> error("Expected a per-operation cipher while seeding the key")
                    })
                })
                aliases.add(originalRequest.keySpec.alias)
                assertFalse(keyStore.containsAlias(originalRequest.keySpec.alias))
                storage.write(entry, plaintext, originalRequest).getOrThrow()
                val ciphertext = requireNotNull(storage.data.first()[entry])
                val originalKey = originalRequest.keySpec.getOrGenerateSecretKey().getOrThrow()
                originalKey.info().assertPerOperationAuthentication()

                var credentialAuthenticationCount = 0
                val currentRequest = KeystoreAccessRequest.ForMnemonic(
                    hasStrongAuthenticator = handler.hasStrongAuthenticator,
                    authorize = { args ->
                        withTimeout(90_000) { handler.askForBiometrics(args.cipher) }.mapCatching { cipher ->
                            credentialAuthenticationCount++
                            Log.i("SargonAuthRegression", "Production PIN authentication succeeded: ${args.javaClass.simpleName}")
                            when (args) {
                                is KeystoreAccessRequest.AuthorizationArgs.Encrypt -> args.copy(cipher = requireNotNull(cipher))
                                is KeystoreAccessRequest.AuthorizationArgs.Decrypt -> args.copy(cipher = requireNotNull(cipher))
                                KeystoreAccessRequest.AuthorizationArgs.TimeWindowAuth -> args
                            }
                        }
                    }
                )
                val result = storage.read(entry, currentRequest)
                assertEquals("The current PIN/password must have been accepted", 1, credentialAuthenticationCount)
                assertEquals(ciphertext, storage.data.first()[entry])
                currentRequest.keySpec.getOrGenerateSecretKey().getOrThrow()
                    .info().assertPerOperationAuthentication()

                if (result.isFailure) {
                    Log.i("SargonAuthRegression", "Production read failed after successful PIN authentication", result.exceptionOrNull())
                    // Rule out lost/corrupt data or an invalidated key: the same PIN and key must
                    // still decrypt this record when authentication includes the cipher.
                    val cipher = EncryptionHelper.initDecryptCipher(
                        android.util.Base64.decode(ciphertext, android.util.Base64.DEFAULT), originalKey
                    ).getOrThrow()
                    val authorizedCipher = requireNotNull(authenticate(activity.await(), cipher))
                    assertEquals(plaintext, ciphertext.decrypt(authorizedCipher).getOrThrow())
                    Log.i("SargonAuthRegression", "Control read with PIN-authorized cipher succeeded")
                }

                assertEquals("A valid device credential must preserve access to the existing mnemonic",
                    plaintext, result.getOrThrow())
                storage.write(entry, "updated disposable seed", currentRequest).getOrThrow()
                assertEquals("updated disposable seed", storage.read(entry, currentRequest).getOrThrow())
                assertEquals("Each read/write must authenticate its own cipher", 3, credentialAuthenticationCount)
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

    private fun Throwable?.isAuthenticationFailure() = generateSequence(this) { it.cause }
        .any { failure ->
            when {
                failure is UserNotAuthenticatedException -> true
                Build.VERSION.SDK_INT >= Build.VERSION_CODES.TIRAMISU ->
                    failure is android.security.KeyStoreException && failure.requiresUserAuthentication()
                // Before API 33, KeyStoreException has no public error-inspection API. The framework
                // maps Keymaster's authentication error to this message, including at Cipher.doFinal.
                else -> failure.javaClass.name == "android.security.KeyStoreException" &&
                    failure.message?.contains("Key user not authenticated", ignoreCase = true) == true
            }
        }
}
