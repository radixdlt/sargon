package com.radixdlt.sargon.samples

import com.radixdlt.sargon.NetworkMethod
import com.radixdlt.sargon.annotation.UsesSampleValues
import com.radixdlt.sargon.newNetworkMethodSample
import com.radixdlt.sargon.newNetworkMethodSampleOther

@UsesSampleValues
val NetworkMethod.Companion.sample: Sample<NetworkMethod>
    get() = object : Sample<NetworkMethod> {

        override fun invoke(): NetworkMethod = newNetworkMethodSample()

        override fun other(): NetworkMethod = newNetworkMethodSampleOther()
    }