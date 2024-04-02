package com.radixdlt.sargon.samples

import com.radixdlt.sargon.annotation.UsesSampleValues
import com.radixdlt.sargon.Gateway
import com.radixdlt.sargon.gatewayMainnet
import com.radixdlt.sargon.gatewayStokenet

@UsesSampleValues
val Gateway.Companion.sampleMainnet: Sample<Gateway>
    get() = object : Sample<Gateway> {

        override fun invoke(): Gateway = gatewayMainnet()

        override fun other(): Gateway = invoke()
    }

@UsesSampleValues
val Gateway.Companion.sampleStokenet: Sample<Gateway>
    get() = object : Sample<Gateway> {

        override fun invoke(): Gateway = gatewayStokenet()

        override fun other(): Gateway = invoke()
    }