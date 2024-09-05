package com.radixdlt.sargon.os.storage

import android.os.Build
import android.security.keystore.KeyGenParameterSpec
import android.security.keystore.KeyPermanentlyInvalidatedException
import android.security.keystore.KeyProperties
import com.radixdlt.sargon.Uuid
import com.radixdlt.sargon.annotation.KoverIgnore
import java.security.KeyStore
import java.security.ProviderException
import javax.crypto.KeyGenerator
import javax.crypto.SecretKey

/**
 * A request to the keystore that describes which [KeySpec] should be used for cryptographic
 * operations. [requestAuthorization] is only invoked for [KeySpec]s that are defined with
 * [KeyGenParameterSpec#Builder#setUserAuthenticationRequired] to true
 */
internal sealed interface KeystoreAccessRequest {

    val keySpec: KeySpec

    suspend fun requestAuthorization(): Result<Unit>

    data object ForProfile: KeystoreAccessRequest {
        override val keySpec: KeySpec = KeySpec.Profile()

        override suspend fun requestAuthorization(): Result<Unit> = Result.success(Unit)
    }

    data object ForRadixConnect: KeystoreAccessRequest {
        override val keySpec: KeySpec = KeySpec.RadixConnect()

        override suspend fun requestAuthorization(): Result<Unit> = Result.success(Unit)
    }

    data class ForMnemonic(
        private val onRequestAuthorization: suspend () -> Result<Unit>
    ): KeystoreAccessRequest {
        override val keySpec: KeySpec = KeySpec.Mnemonic()

        override suspend fun requestAuthorization(): Result<Unit> = onRequestAuthorization()

    }
}

/**
 * The description of the key that describes for cryptographic operations on keystore.
 */
@KoverIgnore
internal sealed class KeySpec(val alias: String) {

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

    open fun delete(): Result<Unit> = runCatching {
        val keyStore = KeyStore.getInstance(PROVIDER).apply { load(null) }
        keyStore.deleteEntry(alias)
    }

    internal fun getSecretKey(): Result<SecretKey?> = runCatching {
        val keyStore = KeyStore.getInstance(PROVIDER).apply { load(null) }
        (keyStore.getEntry(alias, null) as? KeyStore.SecretKeyEntry)?.secretKey
    }

    @KoverIgnore
    class Profile(alias: String = KEY_ALIAS_PROFILE) : KeySpec(alias) {
        override fun generateSecretKey(): Result<SecretKey> = AesKeyGeneratorBuilder(alias = alias)
            .build()
    }

    @KoverIgnore
    class RadixConnect(alias: String = KEY_ALIAS_RADIX_CONNECT): KeySpec(alias) {
        override fun generateSecretKey(): Result<SecretKey> = AesKeyGeneratorBuilder(alias = alias)
            .build()

        override fun delete(): Result<Unit> = runCatching {
            error("KeySpec for RadixConnect should not be deleted")
        }
    }

    @KoverIgnore
    class Mnemonic(
        alias: String = KEY_ALIAS_MNEMONIC,
        private val authenticationTimeoutSeconds: Int = KEY_AUTHORIZATION_TIMEOUT_SECONDS
    ) : KeySpec(alias) {
        override fun generateSecretKey(): Result<SecretKey> = AesKeyGeneratorBuilder(alias = alias)
            .setAuthenticationRequired(authenticationTimeout = authenticationTimeoutSeconds)
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
    private data class AesKeyGeneratorBuilder(
        private val alias: String,
    ) {
        private var authenticationTimeoutSeconds: Int? = null

        fun setAuthenticationRequired(
            authenticationTimeout: Int
        ) = apply {
            require(authenticationTimeout > 0) { "Authentication timeout seconds must be > 0" }
            authenticationTimeoutSeconds = authenticationTimeout
        }

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
    }
}