package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.Mnemonic
import com.radixdlt.sargon.mnemonicPhrase

val Mnemonic.phrase: String
    get() = mnemonicPhrase(from = this)