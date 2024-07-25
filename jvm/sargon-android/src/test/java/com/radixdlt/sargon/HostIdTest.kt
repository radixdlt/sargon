package com.radixdlt.sargon

import com.radixdlt.sargon.samples.Sample
import com.radixdlt.sargon.samples.sample

class HostIdTest: SampleTestable<HostId> {
    override val samples: List<Sample<HostId>>
        get() = listOf(HostId.sample)
}