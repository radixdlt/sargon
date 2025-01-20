package com.radixdlt.sargon.samples

import com.radixdlt.sargon.TimePeriod
import com.radixdlt.sargon.annotation.UsesSampleValues
import com.radixdlt.sargon.newTimePeriodSample
import com.radixdlt.sargon.newTimePeriodSampleOther

@UsesSampleValues
val TimePeriod.Companion.sample: Sample<TimePeriod>
    get() = object : Sample<TimePeriod> {
        override fun invoke(): TimePeriod = newTimePeriodSample()

        override fun other(): TimePeriod = newTimePeriodSampleOther()
    }