package com.radixdlt.sargon

import com.radixdlt.sargon.samples.Sample
import com.radixdlt.sargon.samples.sample

class OffDeviceMnemonicFactorSourceTest: SampleTestable<OffDeviceMnemonicFactorSource> {
    override val samples: List<Sample<OffDeviceMnemonicFactorSource>>
        get() = listOf(OffDeviceMnemonicFactorSource.sample)
}