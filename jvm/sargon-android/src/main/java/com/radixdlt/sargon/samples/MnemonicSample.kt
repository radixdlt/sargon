package com.radixdlt.sargon.samples

import androidx.annotation.VisibleForTesting
import com.radixdlt.sargon.Mnemonic
import com.radixdlt.sargon.newMnemonicSample
import com.radixdlt.sargon.newMnemonicSampleOther

@VisibleForTesting
val Mnemonic.Companion.sample: Sample<Mnemonic>
    get() = object : Sample<Mnemonic> {

        override fun invoke(): Mnemonic = newMnemonicSample()

        override fun other(): Mnemonic = newMnemonicSampleOther()
    }