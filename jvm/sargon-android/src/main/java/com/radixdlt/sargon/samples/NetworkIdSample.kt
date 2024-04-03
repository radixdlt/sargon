package com.radixdlt.sargon.samples

import com.radixdlt.sargon.annotation.UsesSampleValues
import com.radixdlt.sargon.NetworkId

@UsesSampleValues
val NetworkId.Companion.sample: Sample<NetworkId>
    get() = object : Sample<NetworkId> {

        override fun invoke(): NetworkId = NetworkId.MAINNET

        override fun other(): NetworkId = NetworkId.STOKENET
    }