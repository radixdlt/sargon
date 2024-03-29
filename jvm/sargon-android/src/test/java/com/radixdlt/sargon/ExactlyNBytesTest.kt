package com.radixdlt.sargon

import com.radixdlt.sargon.extensions.bytes
import com.radixdlt.sargon.extensions.hex
import com.radixdlt.sargon.extensions.init
import com.radixdlt.sargon.extensions.randomBagOfBytes
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Assertions.assertNotEquals
import org.junit.jupiter.api.Test
import org.junit.jupiter.api.assertDoesNotThrow
import org.junit.jupiter.api.assertThrows

class ExactlyNBytesTest {

    @Test
    fun testEquality() {
        val exactly29Bytes = randomBagOfBytes(byteCount = 29)
        assertEquals(
            Exactly29Bytes.init(bytes = exactly29Bytes),
            Exactly29Bytes.init(bytes = exactly29Bytes)
        )
        assertEquals(exactly29Bytes.hex, Exactly29Bytes.init(exactly29Bytes).hex)
        assertEquals(exactly29Bytes, Exactly29Bytes.init(exactly29Bytes).bytes)
        assertNotEquals(
            Exactly29Bytes.init(bytes = randomBagOfBytes(byteCount = 29)),
            Exactly29Bytes.init(bytes = randomBagOfBytes(byteCount = 29))
        )

        val exactly32Bytes = randomBagOfBytes(byteCount = 32)
        assertEquals(
            Exactly32Bytes.init(bytes = exactly32Bytes),
            Exactly32Bytes.init(bytes = exactly32Bytes)
        )
        assertEquals(exactly32Bytes.hex, Exactly32Bytes.init(exactly32Bytes).hex)
        assertEquals(exactly32Bytes, Exactly32Bytes.init(exactly32Bytes).bytes)
        assertNotEquals(
            Exactly32Bytes.init(bytes = randomBagOfBytes(byteCount = 32)),
            Exactly32Bytes.init(bytes = randomBagOfBytes(byteCount = 32))
        )

        val exactly33Bytes = randomBagOfBytes(byteCount = 33)
        assertEquals(
            Exactly33Bytes.init(bytes = exactly33Bytes),
            Exactly33Bytes.init(bytes = exactly33Bytes)
        )
        assertEquals(exactly33Bytes.hex, Exactly33Bytes.init(exactly33Bytes).hex)
        assertEquals(exactly33Bytes, Exactly33Bytes.init(exactly33Bytes).bytes)
        assertNotEquals(
            Exactly33Bytes.init(bytes = randomBagOfBytes(byteCount = 33)),
            Exactly33Bytes.init(bytes = randomBagOfBytes(byteCount = 33))
        )

        val exactly64Bytes = randomBagOfBytes(byteCount = 64)
        assertEquals(
            Exactly64Bytes.init(bytes = exactly64Bytes),
            Exactly64Bytes.init(bytes = exactly64Bytes)
        )
        assertEquals(exactly64Bytes.hex, Exactly64Bytes.init(exactly64Bytes).hex)
        assertEquals(exactly64Bytes, Exactly64Bytes.init(exactly64Bytes).bytes)
        assertNotEquals(
            Exactly64Bytes.init(bytes = randomBagOfBytes(byteCount = 64)),
            Exactly64Bytes.init(bytes = randomBagOfBytes(byteCount = 64))
        )

        val exactly65Bytes = randomBagOfBytes(byteCount = 65)
        assertEquals(
            Exactly65Bytes.init(bytes = exactly65Bytes),
            Exactly65Bytes.init(bytes = exactly65Bytes)
        )
        assertEquals(exactly65Bytes.hex, Exactly65Bytes.init(exactly65Bytes).hex)
        assertEquals(exactly65Bytes, Exactly65Bytes.init(exactly65Bytes).bytes)
        assertNotEquals(
            Exactly65Bytes.init(bytes = randomBagOfBytes(byteCount = 65)),
            Exactly65Bytes.init(bytes = randomBagOfBytes(byteCount = 65))
        )
    }

    @Test
    fun testExactly29Bytes() {
        assertDoesNotThrow { Exactly29Bytes.init(randomBagOfBytes(byteCount = 29)) }
        assertThrows<CommonException.InvalidByteCount> {
            Exactly29Bytes.init(randomBagOfBytes(byteCount = 30))
        }
    }

    @Test
    fun testExactly32Bytes() {
        assertDoesNotThrow { Exactly32Bytes.init(randomBagOfBytes(byteCount = 32)) }
        assertThrows<CommonException.InvalidByteCount> {
            Exactly32Bytes.init(randomBagOfBytes(byteCount = 33))
        }
    }

    @Test
    fun testExactly33Bytes() {
        assertDoesNotThrow { Exactly33Bytes.init(randomBagOfBytes(byteCount = 33)) }
        assertThrows<CommonException.InvalidByteCount> {
            Exactly33Bytes.init(randomBagOfBytes(byteCount = 34))
        }
    }

    @Test
    fun testExactly64Bytes() {
        assertDoesNotThrow { Exactly64Bytes.init(randomBagOfBytes(byteCount = 64)) }
        assertThrows<CommonException.InvalidByteCount> {
            Exactly64Bytes.init(randomBagOfBytes(byteCount = 65))
        }
    }

    @Test
    fun testExactly65Bytes() {
        assertDoesNotThrow { Exactly65Bytes.init(randomBagOfBytes(byteCount = 65)) }
        assertThrows<CommonException.InvalidByteCount> {
            Exactly65Bytes.init(randomBagOfBytes(byteCount = 66))
        }
    }

}