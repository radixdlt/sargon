package com.radixdlt.sargon.samples

import com.radixdlt.sargon.annotation.UsesSampleValues
import com.radixdlt.sargon.Mnemonic
import com.radixdlt.sargon.newMnemonicSample
import com.radixdlt.sargon.newMnemonicSampleOther

@UsesSampleValues
val Mnemonic.Companion.sample: Sample<Mnemonic>
    get() = object : Sample<Mnemonic> {

        override fun invoke(): Mnemonic = newMnemonicSample()

        override fun other(): Mnemonic = newMnemonicSampleOther()
    }