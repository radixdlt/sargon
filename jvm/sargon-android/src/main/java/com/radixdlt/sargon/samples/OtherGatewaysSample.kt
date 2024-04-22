package com.radixdlt.sargon.samples

import com.radixdlt.sargon.OtherGateways
import com.radixdlt.sargon.annotation.UsesSampleValues
import com.radixdlt.sargon.newOtherGatewaysSample
import com.radixdlt.sargon.newOtherGatewaysSampleOther

@UsesSampleValues
val OtherGateways.Companion.sample: Sample<OtherGateways>
    get() = object : Sample<OtherGateways> {
        override fun invoke(): OtherGateways = newOtherGatewaysSample()

        override fun other(): OtherGateways = newOtherGatewaysSampleOther()
    }
