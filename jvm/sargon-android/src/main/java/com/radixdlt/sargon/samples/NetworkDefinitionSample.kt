package com.radixdlt.sargon.samples

import com.radixdlt.sargon.NetworkDefinition
import com.radixdlt.sargon.annotation.UsesSampleValues
import com.radixdlt.sargon.newNetworkDefinitionSample
import com.radixdlt.sargon.newNetworkDefinitionSampleOther

@UsesSampleValues
val NetworkDefinition.Companion.sample: Sample<NetworkDefinition>
    get() = object : Sample<NetworkDefinition> {
        override fun invoke(): NetworkDefinition = newNetworkDefinitionSample()

        override fun other(): NetworkDefinition = newNetworkDefinitionSampleOther()

    }