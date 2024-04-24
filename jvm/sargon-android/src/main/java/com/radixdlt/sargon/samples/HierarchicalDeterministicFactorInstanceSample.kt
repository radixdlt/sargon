package com.radixdlt.sargon.samples

import com.radixdlt.sargon.HierarchicalDeterministicFactorInstance
import com.radixdlt.sargon.annotation.UsesSampleValues
import com.radixdlt.sargon.newHierarchicalDeterministicFactorInstanceSample
import com.radixdlt.sargon.newHierarchicalDeterministicFactorInstanceSampleOther

@UsesSampleValues
val HierarchicalDeterministicFactorInstance.Companion.sample: Sample<HierarchicalDeterministicFactorInstance>
    get() = object : Sample<HierarchicalDeterministicFactorInstance> {
        override fun invoke(): HierarchicalDeterministicFactorInstance =
            newHierarchicalDeterministicFactorInstanceSample()

        override fun other(): HierarchicalDeterministicFactorInstance =
            newHierarchicalDeterministicFactorInstanceSampleOther()

    }