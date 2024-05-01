package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.Bip39Language
import com.radixdlt.sargon.Bip39Word
import com.radixdlt.sargon.Bip39WordCount
import com.radixdlt.sargon.Entropy16Bytes
import com.radixdlt.sargon.Entropy20Bytes
import com.radixdlt.sargon.Entropy24Bytes
import com.radixdlt.sargon.Entropy28Bytes
import com.radixdlt.sargon.Entropy32Bytes
import com.radixdlt.sargon.Mnemonic
import com.radixdlt.sargon.mnemonicPhrase
import com.radixdlt.sargon.newMnemonicFromPhrase
import com.radixdlt.sargon.newMnemonicFromPhraseLanguage
import com.radixdlt.sargon.newMnemonicFromWords
import com.radixdlt.sargon.newMnemonicGenerateWithEntropy

fun Mnemonic.Companion.init(
    wordCount: Bip39WordCount,
    language: Bip39Language
): Mnemonic {
    val entropy = when (wordCount) {
        Bip39WordCount.TWENTY_FOUR -> Entropy32Bytes.random().asGeneral()
        Bip39WordCount.TWENTY_ONE -> Entropy28Bytes.random().asGeneral()
        Bip39WordCount.EIGHTEEN -> Entropy24Bytes.random().asGeneral()
        Bip39WordCount.FIFTEEN -> Entropy20Bytes.random().asGeneral()
        Bip39WordCount.TWELVE -> Entropy16Bytes.random().asGeneral()
    }

    return newMnemonicGenerateWithEntropy(entropy = entropy, language = language)
}

@Throws(SargonException::class)
fun Mnemonic.Companion.init(phrase: String) = newMnemonicFromPhrase(phrase = phrase)

@Throws(SargonException::class)
fun Mnemonic.Companion.init(phrase: String, language: Bip39Language) =
    newMnemonicFromPhraseLanguage(phrase = phrase, language)

@Throws(SargonException::class)
fun Mnemonic.Companion.init(words: List<Bip39Word>) = newMnemonicFromWords(words = words)


/**
 * Returns the words of a mnemonic as a String joined by spaces, e.g.
 * "zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo wrong"
 */
val Mnemonic.phrase: String
    get() = mnemonicPhrase(from = this)