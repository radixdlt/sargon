package com.radixdlt.sargon.os.storage

import androidx.test.ext.junit.runners.AndroidJUnit4
import androidx.test.filters.SmallTest
import com.radixdlt.sargon.extensions.then
import org.junit.Assert.assertEquals
import org.junit.Assert.assertThrows
import org.junit.Assert.assertTrue
import org.junit.Test
import org.junit.runner.RunWith

@RunWith(AndroidJUnit4::class)
@SmallTest
class EncryptionHelperWithSpecKeyTest {
    @Test
    fun roundtripForString() {
        val message = "A message needing encryption"
        val spec = KeySpec.RadixConnect()

        assertEquals(
            message,
            message.encrypt(keySpec = spec).then {
                it.decrypt(keySpec = spec)
            }.getOrThrow()
        )
    }

    @Test
    fun roundtripForByteArray() {
        val message = "A message needing encryption".toByteArray()
        val spec = KeySpec.RadixConnect()

        assertTrue(
            message.contentEquals(
                message.encrypt(keySpec = spec).then {
                    it.decrypt(keySpec = spec)
                }.getOrThrow()
            )
        )
    }

    @Test
    fun unsupportedTypeEncrypt() {
        val message = 10
        val spec = KeySpec.RadixConnect()

        assertThrows(IllegalArgumentException::class.java) {
            message.encrypt(keySpec = spec).getOrThrow()
        }
    }

    @Test
    fun unsupportedTypeDecrypt() {
        val encrypted = 2048
        val spec = KeySpec.RadixConnect()

        assertThrows(IllegalArgumentException::class.java) {
            encrypted.decrypt(keySpec = spec).getOrThrow()
        }
    }
}