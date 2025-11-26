package com.radixdlt.sargon.os.storage

import android.os.Build
import android.security.keystore.KeyGenParameterSpec
import android.security.keystore.KeyPermanentlyInvalidatedException
import android.security.keystore.KeyProperties
import com.radixdlt.sargon.Uuid
import com.radixdlt.sargon.annotation.KoverIgnore
import com.radixdlt.sargon.extensions.then
import com.radixdlt.sargon.extensions.toUnit
import com.radixdlt.sargon.os.storage.KeySpec.Companion.KEY_ALIAS_MNEMONIC
import com.radixdlt.sargon.os.storage.KeystoreAccessRequest.AuthorizationArgs
import com.radixdlt.sargon.os.storage.KeystoreAccessRequest.Purpose
import timber.log.Timber
import java.security.KeyStore
import java.security.KeyStoreException
import java.security.ProviderException
import javax.crypto.Cipher
import javax.crypto.KeyGenerator
import javax.crypto.SecretKey

/**
 * A request to the keystore that describes which [KeySpec] should be used for cryptographic
 * operations. [requestAuthorization] is only invoked for [KeySpec]s that are defined with
 * [KeyGenParameterSpec#Builder#setUserAuthenticationRequired] to true
 */
sealed interface KeystoreAccessRequest {

    val keySpec: KeySpec

    suspend fun requestAuthorization(): Result<Unit>

    data object ForProfile : KeystoreAccessRequest {

        override val keySpec: KeySpec = KeySpec.Profile()

        override suspend fun requestAuthorization(): Result<Unit> = Result.success(Unit)
    }

    data object ForRadixConnect : KeystoreAccessRequest {

        override val keySpec: KeySpec = KeySpec.RadixConnect()

        override suspend fun requestAuthorization(): Result<Unit> = Result.success(Unit)
    }

    data class ForCache(private val alias: String) : KeystoreAccessRequest {

        override val keySpec: KeySpec = KeySpec.Cache(alias)

        override suspend fun requestAuthorization(): Result<Unit> = Result.success(Unit)
    }

    data class ForMnemonic(
        private val hasStrongAuthenticator: Boolean,
        private val authorize: suspend (AuthorizationArgs) -> Result<AuthorizationArgs>
    ) : KeystoreAccessRequest {

        private val mnemonicKeySpec = KeySpec.Mnemonic(hasStrongAuthenticator = hasStrongAuthenticator)

        override val keySpec: KeySpec = mnemonicKeySpec

        override suspend fun requestAuthorization(): Result<Unit> {
            return authorize(AuthorizationArgs.TimeWindowAuth).toUnit()
        }

        suspend fun requestAuthorization(purpose: Purpose): Result<AuthorizationArgs> {
            return if (!hasStrongAuthenticator) {
                authorize(AuthorizationArgs.TimeWindowAuth)
            } else {
                keySpec.getOrGenerateSecretKey()
                    .then { secretKey ->
                        when (purpose) {
                            is Purpose.OneTimeDecrypt -> EncryptionHelper.initDecryptCipher(
                                encryptedValue = purpose.encryptedValue,
                                secretKey = secretKey
                            ).map { cipher ->
                                AuthorizationArgs.Decrypt(
                                    cipher = cipher
                                )
                            }

                            Purpose.OneTimeEncrypt -> EncryptionHelper.initEncryptCipher(
                                secretKey = secretKey
                            ).map { cipherAndIvBytes ->
                                AuthorizationArgs.Encrypt(
                                    cipher = cipherAndIvBytes.first,
                                    ivBytes = cipherAndIvBytes.second
                                )
                            }
                        }
                    }.then { args ->
                        authorize(args)
                    }
            }
        }
    }

    sealed interface Purpose {

        data object OneTimeEncrypt : Purpose

        data class OneTimeDecrypt(
            val encryptedValue: ByteArray
        ) : Purpose
    }

    sealed class AuthorizationArgs(
        open val cipher: Cipher?
    ) {

        data class Encrypt(
            override val cipher: Cipher,
            val ivBytes: ByteArray
        ) : AuthorizationArgs(cipher)

        data class Decrypt(
            override val cipher: Cipher
        ) : AuthorizationArgs(cipher)

        data object TimeWindowAuth : AuthorizationArgs(null)
    }

    companion object {

        private const val AES_GCM_NOPADDING = "AES/GCM/NoPadding"
        private const val GCM_IV_LENGTH = 12
        private const val AUTH_TAG_LENGTH = 128 // bit
    }
}

/**
 * The description of the key that describes for cryptographic operations on keystore.
 */
@KoverIgnore
sealed class KeySpec(val alias: String) {

    /**
     * The implementation of these methods are heavily based on this:
     * https://gist.github.com/patrickfav/7e28d4eb4bf500f7ee8012c4a0cf7bbf
     * and for a deeper knowledge please read this article:
     * https://levelup.gitconnected.com/doing-aes-gcm-in-android-adventures-in-the-field-72617401269d
     */
    fun getOrGenerateSecretKey(): Result<SecretKey> = getSecretKey()
        .mapCatching { existingSecretKey ->
            existingSecretKey ?: generateSecretKey().getOrThrow()
        }

    internal abstract fun generateSecretKey(): Result<SecretKey>

    internal fun getSecretKey(): Result<SecretKey?> = runCatching {
        val keyStore = KeyStore.getInstance(PROVIDER).apply { load(null) }
        keyStore.getKey(alias, null) as? SecretKey
    }

    @KoverIgnore
    class Profile(alias: String = KEY_ALIAS_PROFILE) : KeySpec(alias) {
        override fun generateSecretKey(): Result<SecretKey> = AesKeyGeneratorBuilder(alias = alias)
            .build()
    }

    @KoverIgnore
    class RadixConnect(alias: String = KEY_ALIAS_RADIX_CONNECT) : KeySpec(alias) {
        override fun generateSecretKey(): Result<SecretKey> = AesKeyGeneratorBuilder(alias = alias)
            .build()
    }

    @KoverIgnore
    class Mnemonic(
        alias: String = KEY_ALIAS_MNEMONIC,
        private val authenticationTimeoutSeconds: Int? = KEY_AUTHORIZATION_TIMEOUT_SECONDS
    ) : KeySpec(alias) {

        constructor(
            alias: String = KEY_ALIAS_MNEMONIC,
            hasStrongAuthenticator: Boolean
        ) : this(
            alias,
            if (hasStrongAuthenticator) {
                // If there is a strong authenticator, we can prevent timeout usage
                // which results in invoking crypto-based authentication,
                // i.e. passing CryptoObject to BiometricPrompt
                null
            } else {
                // If there is no strong authenticator, we cannot use crypto-based authentication
                // so we set a timeout and allow using the key many times within the specified time window
                KEY_AUTHORIZATION_TIMEOUT_SECONDS
            }
        )

        override fun generateSecretKey(): Result<SecretKey> = AesKeyGeneratorBuilder(alias = alias)
            .apply {
                authenticationTimeoutSeconds?.let {
                    setAuthenticationTimeoutSeconds(authenticationTimeout = it)
                }
            }
            .build()

        fun checkIfPermanentlyInvalidated(): Boolean {
            // on pixel 6 pro when lock screen is removed, key entry for an alias is null
            val secretKeyResult = getSecretKey()
            if (secretKeyResult.isFailure || secretKeyResult.getOrNull() == null) return true

            val secretKey = requireNotNull(secretKeyResult.getOrNull())
            val result = Uuid.randomUUID().toString().encrypt(secretKey = secretKey)
            // according to documentation this is exception that should be thrown if we try to use
            // invalidated key, but behavior I saw when removing lock screen is that key is
            // automatically deleted from the keystore
            return result.exceptionOrNull() is KeyPermanentlyInvalidatedException
        }
    }

    @KoverIgnore
    class Cache(alias: String) : KeySpec(alias) {
        override fun generateSecretKey(): Result<SecretKey> = AesKeyGeneratorBuilder(alias = alias)
            .build()
    }

    @KoverIgnore
    private data class AesKeyGeneratorBuilder(
        private val alias: String,
    ) {
        private var authenticationTimeoutSeconds: Int? = null

        /**
         * Sets whether the generated key is authorized to be used only if the user has been authenticated
         * within the given [authenticationTimeout] in seconds.
         */
        fun setAuthenticationTimeoutSeconds(
            authenticationTimeout: Int
        ) = apply {
            require(authenticationTimeout > 0) { "Authentication timeout seconds must be > 0" }
            authenticationTimeoutSeconds = authenticationTimeout
        }

        /**
         * If authenticationTimeoutSeconds is not set, the key will require user authentication for every use.
         * This is recommended for highly sensitive data such as mnemonic storage.
         */
        fun build(): Result<SecretKey> = runCatching {
            val keyGenerator = KeyGenerator.getInstance(KeyProperties.KEY_ALGORITHM_AES, PROVIDER)
            val keygenParameterSpecBuilder = KeyGenParameterSpec.Builder(
                alias,
                KeyProperties.PURPOSE_ENCRYPT or KeyProperties.PURPOSE_DECRYPT
            ).setBlockModes(KeyProperties.BLOCK_MODE_GCM)
                .setEncryptionPaddings(KeyProperties.ENCRYPTION_PADDING_NONE)
                // This is required to be able to provide the IV ourselves
                .setRandomizedEncryptionRequired(false)
                .setKeySize(AES_KEY_SIZE)

            authenticationTimeoutSeconds?.let { timeout ->
                keygenParameterSpecBuilder
                    .setUserAuthenticationRequired(true)
                    .setInvalidatedByBiometricEnrollment(false)

                if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.R) {
                    keygenParameterSpecBuilder.setUserAuthenticationParameters(
                        timeout,
                        KeyProperties.AUTH_BIOMETRIC_STRONG or KeyProperties.AUTH_DEVICE_CREDENTIAL
                    )
                } else {
                    keygenParameterSpecBuilder.setUserAuthenticationValidityDurationSeconds(
                        timeout
                    )
                }

                keygenParameterSpecBuilder.compatSetIsStrongBoxBacked(true)
            }

            try {
                keyGenerator.init(keygenParameterSpecBuilder.build())
                keyGenerator.generateKey()
            } catch (e: ProviderException) {
                keygenParameterSpecBuilder.compatSetIsStrongBoxBacked(false)
                keyGenerator.init(keygenParameterSpecBuilder.build())
                keyGenerator.generateKey()
            }
        }

        private fun KeyGenParameterSpec.Builder.compatSetIsStrongBoxBacked(isBacked: Boolean) = apply {
            if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.P) {
                setIsStrongBoxBacked(isBacked)
            }
        }
    }

    companion object {
        private const val PROVIDER = "AndroidKeyStore"
        private const val AES_KEY_SIZE = 256

        // seem that some low end devices take very long time to generate BDFS mnemonic
        private const val KEY_AUTHORIZATION_TIMEOUT_SECONDS = 30

        private const val KEY_ALIAS_PROFILE = "EncryptedProfileAlias"
        private const val KEY_ALIAS_MNEMONIC = "EncryptedMnemonicAlias"
        private const val KEY_ALIAS_RADIX_CONNECT = "EncryptedRadixConnectSessionAlias"

        /**
         * Resets the given [keySpecs] from [KeyStore]
         *
         * This usually deletes the entry from [KeyStore].
         *
         * In android devices <= 30 we noticed that keys associated to device credentials such
         * as [KEY_ALIAS_MNEMONIC] throw [KeyStoreException] when [KeyStore.deleteEntry] is called,
         * only when the user resets their device credentials to new ones.
         *
         * This made it impossible to associate the same key alias to the new device credentials,
         * resulting to all encrypt/decrypt methods failing. In such cases, the only possible
         * solution is to regenerate a new key with the same alias and associate it with the new
         * device credentials
         */
        fun reset(keySpecs: List<KeySpec>): Result<Unit> = runCatching {
            val keyStore = KeyStore.getInstance(PROVIDER).apply { load(null) }
            keySpecs.forEach {
                try {
                    if (keyStore.containsAlias(it.alias)) {
                        keyStore.deleteEntry(it.alias)
                        Timber.tag("sargon").w("Key spec ${it.alias} deleted successfully")
                    }
                } catch (_: KeyStoreException) {
                    Timber.tag("sargon").w("Deleting key spec ${it.alias} failed. Generating a new one...")
                    // In cases like these the only option is to regenerate the same key
                    it.generateSecretKey().getOrThrow()
                }
            }
        }
    }
}