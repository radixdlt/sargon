package com.radixdlt.sargon

import com.radixdlt.sargon.samples.Sample
import com.radixdlt.sargon.samples.sample

class ThresholdTest: SampleTestable<Threshold> {

    override val samples: List<Sample<Threshold>>
        get() = listOf(Threshold.sample)
}