package com.radixdlt.sargon.samples

import com.radixdlt.sargon.UnsecurifiedHardened
import com.radixdlt.sargon.annotation.UsesSampleValues
import com.radixdlt.sargon.newUnsecurifiedHardenedSample
import com.radixdlt.sargon.newUnsecurifiedHardenedSampleOther

@UsesSampleValues
val UnsecurifiedHardened.Companion.sample: Sample<UnsecurifiedHardened>
    get() = object : Sample<UnsecurifiedHardened> {
        override fun invoke(): UnsecurifiedHardened = newUnsecurifiedHardenedSample()

        override fun other(): UnsecurifiedHardened = newUnsecurifiedHardenedSampleOther()
    }