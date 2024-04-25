package com.radixdlt.sargon.samples

import com.radixdlt.sargon.Bip39WordCount
import com.radixdlt.sargon.annotation.UsesSampleValues

@UsesSampleValues
val Bip39WordCount.Companion.sample: Sample<Bip39WordCount>
    get() = object : Sample<Bip39WordCount> {
        override fun invoke(): Bip39WordCount = Bip39WordCount.TWENTY_FOUR

        override fun other(): Bip39WordCount = Bip39WordCount.TWELVE

    }