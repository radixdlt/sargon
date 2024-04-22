package com.radixdlt.sargon.samples

import com.radixdlt.sargon.SupportedCurves
import com.radixdlt.sargon.annotation.UsesSampleValues
import com.radixdlt.sargon.newSupportedCurvesSample
import com.radixdlt.sargon.newSupportedCurvesSampleOther

@UsesSampleValues
val SupportedCurves.Companion.sample: Sample<SupportedCurves>
    get() = object : Sample<SupportedCurves> {
        override fun invoke(): SupportedCurves = newSupportedCurvesSample()

        override fun other(): SupportedCurves = newSupportedCurvesSampleOther()

    }