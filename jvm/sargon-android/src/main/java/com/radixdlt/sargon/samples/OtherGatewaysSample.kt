package com.radixdlt.sargon.samples

import com.radixdlt.sargon.Gateway
import com.radixdlt.sargon.OtherGateways
import com.radixdlt.sargon.annotation.UsesSampleValues
import com.radixdlt.sargon.extensions.init
import com.radixdlt.sargon.extensions.mainnet
import com.radixdlt.sargon.extensions.stokenet

@UsesSampleValues
val OtherGateways.Companion.sample: Sample<OtherGateways>
    get() = object : Sample<OtherGateways> {
        override fun invoke(): OtherGateways = OtherGateways.init(
            Gateway.mainnet,
            Gateway.stokenet
        )

        override fun other(): OtherGateways = OtherGateways.init(
            Gateway.mainnet
        )
    }
