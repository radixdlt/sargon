package com.radixdlt.sargon.samples

import com.radixdlt.sargon.HierarchicalDeterministicPublicKey
import com.radixdlt.sargon.annotation.UsesSampleValues
import com.radixdlt.sargon.newHierarchicalDeterministicPublicKeySample
import com.radixdlt.sargon.newHierarchicalDeterministicPublicKeySampleOther

@UsesSampleValues
val HierarchicalDeterministicPublicKey.Companion.sample: Sample<HierarchicalDeterministicPublicKey>
    get() = object : Sample<HierarchicalDeterministicPublicKey> {
        override fun invoke(): HierarchicalDeterministicPublicKey =
            newHierarchicalDeterministicPublicKeySample()

        override fun other(): HierarchicalDeterministicPublicKey =
            newHierarchicalDeterministicPublicKeySampleOther()
    }