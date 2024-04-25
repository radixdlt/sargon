package com.radixdlt.sargon.samples

import com.radixdlt.sargon.PrivateHierarchicalDeterministicFactorSource
import com.radixdlt.sargon.annotation.UsesSampleValues
import com.radixdlt.sargon.newPrivateHdFactorSourceSample
import com.radixdlt.sargon.newPrivateHdFactorSourceSampleOther

@UsesSampleValues
val PrivateHierarchicalDeterministicFactorSource.Companion.sample:
        Sample<PrivateHierarchicalDeterministicFactorSource>
    get() = object : Sample<PrivateHierarchicalDeterministicFactorSource> {
        override fun invoke(): PrivateHierarchicalDeterministicFactorSource =
            newPrivateHdFactorSourceSample()

        override fun other(): PrivateHierarchicalDeterministicFactorSource =
            newPrivateHdFactorSourceSampleOther()

    }