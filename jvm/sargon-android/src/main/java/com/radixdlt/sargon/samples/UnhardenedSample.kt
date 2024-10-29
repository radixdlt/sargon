package com.radixdlt.sargon.samples

import com.radixdlt.sargon.Unhardened
import com.radixdlt.sargon.annotation.UsesSampleValues
import com.radixdlt.sargon.newUnhardenedSample
import com.radixdlt.sargon.newUnhardenedSampleOther

@UsesSampleValues
val Unhardened.Companion.sample: Sample<Unhardened>
    get() = object : Sample<Unhardened> {
        override fun invoke(): Unhardened = newUnhardenedSample()

        override fun other(): Unhardened = newUnhardenedSampleOther()
    }