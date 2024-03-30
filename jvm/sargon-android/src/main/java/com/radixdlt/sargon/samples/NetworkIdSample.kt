package com.radixdlt.sargon.samples

import androidx.annotation.VisibleForTesting
import com.radixdlt.sargon.NetworkId

@VisibleForTesting
val NetworkId.Companion.sample: Sample<NetworkId>
    get() = object : Sample<NetworkId> {

        override fun invoke(): NetworkId = NetworkId.MAINNET

        override fun other(): NetworkId = NetworkId.STOKENET
    }