package com.radixdlt.sargon

import com.radixdlt.sargon.samples.Sample
import com.radixdlt.sargon.samples.sample

class PasswordFactorSourceTest: SampleTestable<PasswordFactorSource> {
    override val samples: List<Sample<PasswordFactorSource>>
        get() = listOf(PasswordFactorSource.sample)
}