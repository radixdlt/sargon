package com.radixdlt.sargon.samples

import com.radixdlt.sargon.SecurityStructureOfFactorInstances
import com.radixdlt.sargon.annotation.UsesSampleValues
import com.radixdlt.sargon.newSecurityStructureOfFactorInstancesSample
import com.radixdlt.sargon.newSecurityStructureOfFactorInstancesSampleOther

@UsesSampleValues
val SecurityStructureOfFactorInstances.Companion.sample: Sample<SecurityStructureOfFactorInstances>
    get() = object: Sample<SecurityStructureOfFactorInstances> {
        override fun invoke(): SecurityStructureOfFactorInstances
            = newSecurityStructureOfFactorInstancesSample()

        override fun other(): SecurityStructureOfFactorInstances
            = newSecurityStructureOfFactorInstancesSampleOther()
    }