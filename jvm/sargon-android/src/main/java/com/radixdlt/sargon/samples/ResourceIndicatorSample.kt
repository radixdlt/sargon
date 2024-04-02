package com.radixdlt.sargon.samples

import com.radixdlt.sargon.ResourceIndicator
import com.radixdlt.sargon.annotation.UsesSampleValues
import com.radixdlt.sargon.newResourceIndicatorSample
import com.radixdlt.sargon.newResourceIndicatorSampleOther

@UsesSampleValues
val ResourceIndicator.Companion.sample: Sample<ResourceIndicator>
    get() = object: Sample<ResourceIndicator> {
        override fun invoke(): ResourceIndicator = newResourceIndicatorSample()

        override fun other(): ResourceIndicator = newResourceIndicatorSampleOther()
    }