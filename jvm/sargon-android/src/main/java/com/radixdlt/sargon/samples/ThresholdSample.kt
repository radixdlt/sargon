package com.radixdlt.sargon.samples

import com.radixdlt.sargon.Threshold
import com.radixdlt.sargon.annotation.UsesSampleValues
import com.radixdlt.sargon.newThresholdSample
import com.radixdlt.sargon.newThresholdSampleOther

@UsesSampleValues
val Threshold.Companion.sample: Sample<Threshold>
    get() = object : Sample<Threshold> {
        override fun invoke(): Threshold = newThresholdSample()

        override fun other(): Threshold = newThresholdSampleOther()
    }