package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.Bip39Language
import com.radixdlt.sargon.Bip39Word
import com.radixdlt.sargon.bip39LanguageWordlist

val Bip39Language.wordList: List<Bip39Word>
    get() = bip39LanguageWordlist(language = this)