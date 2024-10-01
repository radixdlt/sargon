@file:OptIn(ExperimentalEncodingApi::class)

package com.radixdlt.sargon.os.storage

import android.util.Base64
import com.radixdlt.sargon.extensions.then
import java.nio.ByteBuffer
import java.nio.charset.StandardCharsets
import java.security.SecureRandom
import java.security.spec.AlgorithmParameterSpec
import javax.crypto.Cipher
import javax.crypto.SecretKey
import javax.crypto.spec.GCMParameterSpec
import javax.crypto.spec.SecretKeySpec
import kotlin.io.encoding.ExperimentalEncodingApi

internal object EncryptionHelper {
    fun encrypt(
        input: ByteArray,
        secretKey: SecretKey
    ): Result<ByteArray> {
        return runCatching {
            val cipher = Cipher.getInstance(AES_GCM_NOPADDING)
            val ivBytes = ByteArray(GCM_IV_LENGTH)
            SecureRandom().nextBytes(ivBytes)
            val parameterSpec = GCMParameterSpec(AUTH_TAG_LENGTH, ivBytes)
            cipher.init(Cipher.ENCRYPT_MODE, secretKey, parameterSpec)

            val ciphertext: ByteArray = cipher.doFinal(input)

            val byteBuffer = ByteBuffer.allocate(ivBytes.size + ciphertext.size)
            byteBuffer.put(ivBytes)
            byteBuffer.put(ciphertext)

            byteBuffer.array()
        }
    }

    fun decrypt(
        input: ByteArray,
        secretKey: SecretKey
    ): Result<ByteArray> {
        return runCatching {
            val cipher = Cipher.getInstance(AES_GCM_NOPADDING)
            val gcmIv: AlgorithmParameterSpec =
                GCMParameterSpec(AUTH_TAG_LENGTH, input, 0, GCM_IV_LENGTH)
            cipher.init(Cipher.DECRYPT_MODE, secretKey, gcmIv)
            cipher.doFinal(input, GCM_IV_LENGTH, input.size - GCM_IV_LENGTH)
        }
    }

    const val AES_ALGORITHM = "AES"
    private const val AES_GCM_NOPADDING = "AES/GCM/NoPadding"
    private const val GCM_IV_LENGTH = 12
    private const val AUTH_TAG_LENGTH = 128 // bit
}

/**
 * Encrypts this value with the provided [secretKey].
 *
 * The receiver must be either a [String] or a [ByteArray]. Other types are not supported as
 * of this moment.
 */
@Suppress("UNCHECKED_CAST")
internal fun <T : Any> T.encrypt(secretKey: SecretKey): Result<T> = runCatching {
    when (this) {
        is String -> EncryptionHelper.encrypt(
            input = toByteArray(),
            secretKey = secretKey
        ).mapCatching {
            Base64.encodeToString(it, Base64.DEFAULT) as T
        }.getOrThrow()

        is ByteArray -> EncryptionHelper.encrypt(input = this, secretKey = secretKey)
            .getOrThrow() as T

        else -> throw IllegalArgumentException(
            "Encrypting ${this::class.java} type is not supported"
        )
    }
}

/**
 * Decrypts this value with the provided [secretKey].
 *
 * The receiver must be either a [String] or a [ByteArray]. Other types are not supported as
 * of this moment.
 */
@Suppress("UNCHECKED_CAST")
internal fun <T : Any> T.decrypt(secretKey: SecretKey): Result<T> = runCatching {
    when (this) {
        is String -> String(
            EncryptionHelper.decrypt(
                input = Base64.decode(
                    this,
                    Base64.DEFAULT
                ),
                secretKey = secretKey
            ).getOrThrow(),
            StandardCharsets.UTF_8
        ) as T

        is ByteArray -> EncryptionHelper.decrypt(input = this, secretKey = secretKey)
            .getOrThrow() as T

        else -> throw IllegalArgumentException(
            "Encrypting ${this::class.java} type is not supported"
        )
    }
}

/**
 * Encrypts this value with the provided [keySpec].
 *
 * The receiver must be either a [String] or a [ByteArray]. Other types are not supported as
 * of this moment.
 */
fun <T : Any> T.encrypt(keySpec: KeySpec) = keySpec.getOrGenerateSecretKey()
    .then { encrypt(secretKey = it) }

/**
 * Decrypts this value with the provided [keySpec].
 *
 * The receiver must be either a [String] or a [ByteArray]. Other types are not supported as
 * of this moment.
 */
fun <T : Any> T.decrypt(keySpec: KeySpec) = keySpec.getOrGenerateSecretKey()
    .then { decrypt(secretKey = it) }

/**
 * Encrypts this value with the provided [encryptionKey].
 *
 * The receiver must be either a [String] or a [ByteArray]. Other types are not supported as
 * of this moment.
 */
fun <T : Any> T.encrypt(encryptionKey: ByteArray) = encrypt(
    secretKey = SecretKeySpec(encryptionKey, EncryptionHelper.AES_ALGORITHM)
)

/**
 * Decrypts this value with the provided [encryptionKey].
 *
 * The receiver must be either a [String] or a [ByteArray]. Other types are not supported as
 * of this moment.
 */
fun <T : Any> T.decrypt(encryptionKey: ByteArray) = decrypt(
    secretKey = SecretKeySpec(encryptionKey, EncryptionHelper.AES_ALGORITHM)
)

