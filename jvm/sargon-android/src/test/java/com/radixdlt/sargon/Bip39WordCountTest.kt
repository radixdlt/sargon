package com.radixdlt.sargon

import com.radixdlt.sargon.extensions.init
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Test

class Bip39WordCountTest {

    @Test
    fun testInit() {
        assertEquals(Bip39WordCount.TWELVE, Bip39WordCount.init(wordCount = 12))
    }

    @Test
    fun testInitFail() {
        val error = runCatching { Bip39WordCount.init(wordCount = 11) }
            .exceptionOrNull()

        assertEquals(11.toULong(), (error as CommonException.InvalidBip39WordCount).badValue)
    }

}