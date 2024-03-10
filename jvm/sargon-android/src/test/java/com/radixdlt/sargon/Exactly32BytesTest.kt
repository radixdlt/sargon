package com.radixdlt.sargon

import com.radixdlt.sargon.extensions.init
import com.radixdlt.sargon.extensions.randomBagOfBytes
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Test

class Exactly32BytesTest {

    @Test
    fun test() {
        val bytes = randomBagOfBytes(byteCount = 32)
        assertEquals(Exactly32Bytes.init(bytes = bytes), Exactly32Bytes.init(bytes = bytes))
    }

}