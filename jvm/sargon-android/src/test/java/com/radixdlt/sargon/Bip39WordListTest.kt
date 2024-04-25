package com.radixdlt.sargon

import com.radixdlt.sargon.extensions.wordList
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Test

class Bip39WordListTest {

    @Test
    fun testLanguageWordCount() {
        assertEquals(2048, Bip39Language.ENGLISH.wordList.size)
    }

}