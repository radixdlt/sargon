package com.radixdlt.sargon

import com.radixdlt.sargon.extensions.phrase
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Test

class MnemonicTest {

    @Test
    fun test() {
        val mnemonic = Mnemonic(
            words = List(11) {
                Bip39Word("zoo", U11(2048.toUShort()), Bip39Language.ENGLISH)
            } + listOf(
                Bip39Word("wrong", U11(2038.toUShort()), Bip39Language.ENGLISH)
            ),
            wordCount = Bip39WordCount.TWELVE,
            language = Bip39Language.ENGLISH
        )

        assertEquals("zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo wrong", mnemonic.phrase)
    }

}