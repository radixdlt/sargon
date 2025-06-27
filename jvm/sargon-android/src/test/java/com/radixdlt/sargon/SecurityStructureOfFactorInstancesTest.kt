package com.radixdlt.sargon

import com.radixdlt.sargon.samples.Sample
import com.radixdlt.sargon.samples.sample

class SecurityStructureOfFactorInstancesTest: SampleTestable<SecurityStructureOfFactorInstances> {
    override val samples: List<Sample<SecurityStructureOfFactorInstances>>
        get() = listOf(SecurityStructureOfFactorInstances.sample)
}