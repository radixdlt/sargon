package com.radixdlt.sargon

import com.radixdlt.sargon.samples.Sample
import com.radixdlt.sargon.samples.sample

class ArculusCardFactorSourceTest: SampleTestable<ArculusCardFactorSource> {
    override val samples: List<Sample<ArculusCardFactorSource>>
        get() = listOf(ArculusCardFactorSource.sample)
}