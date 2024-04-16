package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.Bip39Language
import com.radixdlt.sargon.Bip39Word
import com.radixdlt.sargon.Mnemonic
import com.radixdlt.sargon.mnemonicPhrase
import com.radixdlt.sargon.newMnemonicFromPhrase
import com.radixdlt.sargon.newMnemonicFromPhraseLanguage
import com.radixdlt.sargon.newMnemonicFromWords

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