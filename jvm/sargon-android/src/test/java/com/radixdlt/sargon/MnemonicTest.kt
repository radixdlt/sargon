package com.radixdlt.sargon

import com.radixdlt.sargon.extensions.init
import com.radixdlt.sargon.extensions.phrase
import com.radixdlt.sargon.extensions.randomBagOfBytes
import com.radixdlt.sargon.samples.Sample
import com.radixdlt.sargon.samples.sample
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Test
import org.junit.jupiter.api.assertThrows

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

    @Test
    fun testFromPhrase() {
        val phrase = "zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo wrong"

        assertEquals(
            phrase,
            Mnemonic.init(phrase).phrase,
        )
    }

    @Test
    fun testFromPhraseLanguage() {
        val phrase = "zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo wrong"

        assertEquals(
            phrase,
            Mnemonic.init(phrase, Bip39Language.ENGLISH).phrase,
        )
    }

    @Test
    fun testFromWords() {
        val words = List(11) {
            Bip39Word("zoo", U11(2048.toUShort()), Bip39Language.ENGLISH)
        } + listOf(
            Bip39Word("wrong", U11(2038.toUShort()), Bip39Language.ENGLISH)
        )

        assertEquals(
            words.joinToString(separator = " ") { it.word },
            Mnemonic.init(words).phrase,
        )
    }

    @Test
    fun testNewFromGeneratedEntropy() {
        val language = Bip39Language.ENGLISH
        val mnemonicsCount = 100
        Bip39WordCount.entries.forEach { wordCount ->
            val mnemonics = List(mnemonicsCount) {
                val sut = Mnemonic.init(wordCount = wordCount, language = language)
                assertEquals(Mnemonic.init(phrase = sut.phrase, language = language), sut)
                sut
            }
            assertEquals(mnemonicsCount, mnemonics.toSet().size)
        }
    }

    @Test
    fun testEntropyBytesThrowsWrongSize() {
        assertThrows<CommonException.InvalidByteCount> {
            Entropy32Bytes.init(randomBagOfBytes(36))
        }
        assertThrows<CommonException.InvalidByteCount> {
            Entropy32Bytes.init(randomBagOfBytes(28))
        }

        assertThrows<CommonException.InvalidByteCount> {
            Entropy28Bytes.init(randomBagOfBytes(32))
        }
        assertThrows<CommonException.InvalidByteCount> {
            Entropy28Bytes.init(randomBagOfBytes(24))
        }

        assertThrows<CommonException.InvalidByteCount> {
            Entropy24Bytes.init(randomBagOfBytes(28))
        }
        assertThrows<CommonException.InvalidByteCount> {
            Entropy24Bytes.init(randomBagOfBytes(20))
        }

        assertThrows<CommonException.InvalidByteCount> {
            Entropy20Bytes.init(randomBagOfBytes(24))
        }
        assertThrows<CommonException.InvalidByteCount> {
            Entropy20Bytes.init(randomBagOfBytes(16))
        }

        assertThrows<CommonException.InvalidByteCount> {
            Entropy16Bytes.init(randomBagOfBytes(20))
        }
        assertThrows<CommonException.InvalidByteCount> {
            Entropy16Bytes.init(randomBagOfBytes(12))
        }
    }


}