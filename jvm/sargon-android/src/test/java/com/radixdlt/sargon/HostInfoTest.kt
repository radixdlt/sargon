package com.radixdlt.sargon

import com.radixdlt.sargon.samples.Sample
import com.radixdlt.sargon.samples.sample

class HostInfoTest: SampleTestable<HostInfo> {
    override val samples: List<Sample<HostInfo>>
        get() = listOf(HostInfo.sample)
}