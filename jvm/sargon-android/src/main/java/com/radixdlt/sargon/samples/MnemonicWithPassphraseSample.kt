package com.radixdlt.sargon.samples

import com.radixdlt.sargon.MnemonicWithPassphrase
import com.radixdlt.sargon.annotation.UsesSampleValues
import com.radixdlt.sargon.newMnemonicWithPassphraseSample
import com.radixdlt.sargon.newMnemonicWithPassphraseSampleOther

@UsesSampleValues
val MnemonicWithPassphrase.Companion.sample: Sample<MnemonicWithPassphrase>
    get() = object : Sample<MnemonicWithPassphrase> {
        override fun invoke(): MnemonicWithPassphrase = newMnemonicWithPassphraseSample()

        override fun other(): MnemonicWithPassphrase = newMnemonicWithPassphraseSampleOther()

    }