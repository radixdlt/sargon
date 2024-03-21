package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.Mnemonic
import com.radixdlt.sargon.mnemonicPhrase

/**
 * Returns the words of a mnemonic as a String joined by spaces, e.g.
 * "zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo wrong"
 */
val Mnemonic.phrase: String
    get() = mnemonicPhrase(from = this)