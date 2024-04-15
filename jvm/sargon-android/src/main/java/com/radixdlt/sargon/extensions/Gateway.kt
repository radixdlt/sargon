package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.Gateway
import com.radixdlt.sargon.gatewayMainnet
import com.radixdlt.sargon.gatewayStokenet

val Gateway.Companion.mainnet: Gateway
    get() = gatewayMainnet()

val Gateway.Companion.stokenet: Gateway
    get() = gatewayStokenet()