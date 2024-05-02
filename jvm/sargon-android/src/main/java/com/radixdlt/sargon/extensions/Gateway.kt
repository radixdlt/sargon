package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.Gateway
import com.radixdlt.sargon.Gateways
import com.radixdlt.sargon.NetworkId
import com.radixdlt.sargon.gatewayIsWellknown
import com.radixdlt.sargon.gatewayMainnet
import com.radixdlt.sargon.gatewayStokenet
import com.radixdlt.sargon.gatewayToString
import com.radixdlt.sargon.gatewayWellknownGateways
import com.radixdlt.sargon.newGatewayForNetworkId
import com.radixdlt.sargon.newGatewayWithUrlOnNetwork

val Gateway.Companion.mainnet: Gateway
    get() = gatewayMainnet()

val Gateway.Companion.stokenet: Gateway
    get() = gatewayStokenet()

val Gateway.Companion.wellKnown: Gateways
    get() = gatewayWellknownGateways()

@Throws(SargonException::class)
fun Gateway.Companion.init(url: String, networkId: NetworkId): Gateway =
    newGatewayWithUrlOnNetwork(url = url, networkId = networkId)

fun Gateway.Companion.forNetwork(networkId: NetworkId): Gateway =
    newGatewayForNetworkId(networkId = networkId)

/**
 * Returns the [Gateway]'s url as [String]
 */
val Gateway.string: String
    get() = gatewayToString(gateway = this)

val Gateway.isWellKnown: Boolean
    get() = gatewayIsWellknown(gateway = this)
