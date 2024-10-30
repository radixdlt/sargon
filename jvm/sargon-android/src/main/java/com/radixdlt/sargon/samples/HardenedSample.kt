package com.radixdlt.sargon.samples

import com.radixdlt.sargon.Hardened
import com.radixdlt.sargon.annotation.UsesSampleValues
import com.radixdlt.sargon.newHardenedSample
import com.radixdlt.sargon.newHardenedSampleOther

@UsesSampleValues
val Hardened.Companion.sample: Sample<Hardened>
    get() = object : Sample<Hardened> {
        override fun invoke(): Hardened = newHardenedSample()

        override fun other(): Hardened = newHardenedSampleOther()
    }