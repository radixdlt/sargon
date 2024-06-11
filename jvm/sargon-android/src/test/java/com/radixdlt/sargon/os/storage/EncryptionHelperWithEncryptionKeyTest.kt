package com.radixdlt.sargon.os.storage

import com.radixdlt.sargon.extensions.randomBagOfBytes
import com.radixdlt.sargon.extensions.then
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Assertions.assertNotEquals
import org.junit.jupiter.api.Assertions.assertTrue
import org.junit.jupiter.api.Test
import org.junit.jupiter.api.assertThrows

@OptIn(ExperimentalUnsignedTypes::class, ExperimentalStdlibApi::class)
class EncryptionHelperWithEncryptionKeyTest {

    @Test
    fun `decrypt with AES GCM NoPadding`() {
        val encryptedMessageInHex =
            "6ea80ead36e3fc4f1ad75134776c26534e73086e93f6b3cd7fdbbe390ed428b5c2f0150fd3f16c928e968497060b39ec61660704"
        val encryptionKey = ByteArray(32) { 0xab.toByte() }
        assertEquals(
            "abababababababababababababababababababababababababababababababab",
            encryptionKey.toHexString()
        )


        val decrypted = encryptedMessageInHex.hexToByteArray().decrypt(
            encryptionKey = encryptionKey
        )

        assertEquals(
            "Hello Android from Swift",
            String(decrypted.getOrThrow())
        )
    }

    @Test
    fun `ensure that encrypting the same message does not give the same encrypted output`() {
        val encryptionKeyByteArray = ByteArray(32) { 0xab.toByte() }

        val decryptedMessage = "Hello Android from Swift"
        val encryptedMessage1 = decryptedMessage.encrypt(
            encryptionKey = encryptionKeyByteArray
        ).getOrThrow()

        val encryptedMessage2 = decryptedMessage.encrypt(
            encryptionKey = encryptionKeyByteArray
        ).getOrThrow()

        assertNotEquals(encryptedMessage1, encryptedMessage2)
        assertTrue(
            decryptedMessage.contentEquals(
                encryptedMessage1.decrypt(encryptionKey = encryptionKeyByteArray).getOrThrow()
            ),
        )
        assertTrue(
            decryptedMessage.contentEquals(
                encryptedMessage2.decrypt(encryptionKey = encryptionKeyByteArray).getOrThrow()
            ),
        )
    }

    @Test
    fun roundtripForString() {
        val message = "A message needing encryption"
        val encryptionKey = randomBagOfBytes(byteCount = 32).toUByteArray().toByteArray()

        assertEquals(
            message,
            message.encrypt(encryptionKey = encryptionKey).then {
                it.decrypt(encryptionKey = encryptionKey)
            }.getOrThrow()
        )
    }

    @Test
    fun roundtripForByteArray() {
        val message = "A message needing encryption".toByteArray()
        val encryptionKey = randomBagOfBytes(byteCount = 32).toUByteArray().toByteArray()

        assertTrue(
            message.contentEquals(
                message.encrypt(encryptionKey = encryptionKey).then {
                    it.decrypt(encryptionKey = encryptionKey)
                }.getOrThrow()
            )
        )
    }

    @Test
    fun unsupportedTypeEncrypt() {
        val message = 10
        val encryptionKey = randomBagOfBytes(byteCount = 32).toUByteArray().toByteArray()

        assertThrows<UnsupportedOperationException> {
            message.encrypt(encryptionKey = encryptionKey).getOrThrow()
        }
    }

    @Test
    fun unsupportedTypeDecrypt() {
        val encryptionKey = randomBagOfBytes(byteCount = 32).toUByteArray().toByteArray()
        val encrypted = 2048

        assertThrows<UnsupportedOperationException> {
            encrypted.decrypt(encryptionKey = encryptionKey).getOrThrow()
        }
    }
}