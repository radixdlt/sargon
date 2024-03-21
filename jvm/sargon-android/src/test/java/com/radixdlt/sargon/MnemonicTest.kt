package com.radixdlt.sargon

import com.radixdlt.sargon.extensions.phrase
import com.radixdlt.sargon.samples.Sample
import com.radixdlt.sargon.samples.sample
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Test

class MnemonicTest: SampleTestable<Mnemonic> {

    override val samples: List<Sample<Mnemonic>>
        get() = listOf(Mnemonic.sample)

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