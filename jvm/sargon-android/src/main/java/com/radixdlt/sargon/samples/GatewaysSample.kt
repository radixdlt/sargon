package com.radixdlt.sargon.samples

import com.radixdlt.sargon.annotation.UsesSampleValues
import com.radixdlt.sargon.Gateways
import com.radixdlt.sargon.newGatewaysSample
import com.radixdlt.sargon.newGatewaysSampleOther

@UsesSampleValues
val Gateways.Companion.sample: Sample<Gateways>
    get() = object : Sample<Gateways> {

        override fun invoke(): Gateways = newGatewaysSample()

        override fun other(): Gateways = newGatewaysSampleOther()
    }