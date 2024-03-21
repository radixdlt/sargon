package com.radixdlt.sargon

import com.radixdlt.sargon.samples.Sample
import com.radixdlt.sargon.samples.sample

class HeaderTest: SampleTestable<Header> {

    override val samples: List<Sample<Header>>
        get() = listOf(Header.sample)

}