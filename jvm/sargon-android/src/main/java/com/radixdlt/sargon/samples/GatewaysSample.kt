package com.radixdlt.sargon.samples

import androidx.annotation.VisibleForTesting
import com.radixdlt.sargon.Gateways
import com.radixdlt.sargon.newGatewaysSample
import com.radixdlt.sargon.newGatewaysSampleOther

@VisibleForTesting
val Gateways.Companion.sample: Sample<Gateways>
    get() = object : Sample<Gateways> {

        override fun invoke(): Gateways = newGatewaysSample()

        override fun other(): Gateways = newGatewaysSampleOther()
    }