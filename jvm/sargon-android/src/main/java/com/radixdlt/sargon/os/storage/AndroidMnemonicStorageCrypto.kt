package com.radixdlt.sargon.os.storage

import com.radixdlt.sargon.os.driver.BiometricAuthorizationDriver

/**
 * Adapts [KeySpec], [KeystoreAccessRequest.ForMnemonic] and [EncryptionHelper] for [MnemonicStorage].
 * [MnemonicStorage] owns persisted records and migration commits.
 *
 * Inspects key presence and authentication policy and requests authorization for every read/write,
 * including reads of legacy records whose keys do not require authentication. Protected keys use
 * their existing per-operation or timed authorization policy.
 * Read/write key lookup errors propagate without replacing keys or changing stored records.
 *
 * Normal writes create keys through [ensureKeyExists], when explicitly allowed by the caller. Encryption and
 * decryption require an existing key so retained ciphertext never causes silent key replacement.
 * Explicit recovery uses the original [KeySpec.reset] policy, including its replacement fallback.
 * [MnemonicStorage] runs these operations off Main and coordinates key deletion with migration.
 */
internal class AndroidMnemonicStorageCrypto(private val authorization: BiometricAuthorizationDriver) : MnemonicStorageCrypto {
    override fun keyState(alias: String): MnemonicKeyState {
        val info = KeySpec.Mnemonic(alias).getKeyInfo()
        return when {
            info == null -> MnemonicKeyState.Missing
            info.isUserAuthenticationRequired -> MnemonicKeyState.Protected
            else -> MnemonicKeyState.Unprotected
        }
    }

    override fun isInvalidated(alias: String): Boolean = KeySpec.Mnemonic(alias).checkIfPermanentlyInvalidated()

    override fun reset(alias: String) {
        KeySpec.reset(listOf(KeySpec.Mnemonic(alias).withAuthenticationPolicy(authorization.hasStrongAuthenticator)))
            .getOrThrow()
    }

    override fun ensureKeyExists(alias: String, mayCreate: Boolean) {
        val spec = KeySpec.Mnemonic(alias)
        if (spec.getSecretKey().getOrThrow() != null) return
        check(mayCreate) { "Mnemonic encryption key is missing" }
        spec
            .withAuthenticationPolicy(authorization.hasStrongAuthenticator)
            .generateSecretKey().getOrThrow()
    }

    override fun delete(alias: String) {
        KeySpec.Mnemonic(alias).deleteSecretKey().getOrThrow()
    }

    override suspend fun encrypt(plaintext: String, alias: String): String =
        request(KeySpec.Mnemonic(alias)).encryptWithAuthorization(plaintext).getOrThrow()

    override suspend fun decrypt(ciphertext: String, alias: String): String =
        request(KeySpec.Mnemonic(alias)).decryptWithAuthorization(ciphertext).getOrThrow()

    private fun request(spec: KeySpec.Mnemonic) = KeystoreAccessRequest.ForMnemonic(
        hasStrongAuthenticator = authorization.hasStrongAuthenticator,
        authorize = { args -> authorization.authorize(args.cipher).mapCatching { cipher ->
            when (args) {
                is KeystoreAccessRequest.AuthorizationArgs.Encrypt -> args.copy(cipher = requireNotNull(cipher))
                is KeystoreAccessRequest.AuthorizationArgs.Decrypt -> args.copy(cipher = requireNotNull(cipher))
                KeystoreAccessRequest.AuthorizationArgs.TimeWindowAuth -> args
            }
        } },
        mnemonicKeySpec = spec
    )
}
