package com.radixdlt.sargon

import com.radixdlt.sargon.extensions.init
import com.radixdlt.sargon.extensions.randomBagOfBytes
import org.junit.jupiter.api.Test
import org.junit.jupiter.api.assertDoesNotThrow
import org.junit.jupiter.api.assertThrows

class NonEmptyMax64BytesTest {

    @Test
    fun test() {
        assertDoesNotThrow {
            NonEmptyMax64Bytes.init(randomBagOfBytes(byteCount = 64))
        }

        assertThrows<CommonException.TooManyBytes> {
            NonEmptyMax64Bytes.init(randomBagOfBytes(byteCount = 65))
        }

        assertThrows<CommonException.BytesEmpty> {
            NonEmptyMax64Bytes.init(randomBagOfBytes(byteCount = 0))
        }
    }

}