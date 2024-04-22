package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.Gateway
import com.radixdlt.sargon.Gateways
import com.radixdlt.sargon.gatewaysGetAllElements
import com.radixdlt.sargon.newGateways
import com.radixdlt.sargon.newGatewaysChangingCurrent
import com.radixdlt.sargon.newGatewaysDefault

/**
 * Constructs [Gateways] with [current] set as active Gateway.
 */
fun Gateways.Companion.init(current: Gateway) = newGateways(current = current)

/**
 * The default configuration for each new user:
 * Current: mainnet
 * Other: [ stokenet ]
 */
val Gateways.Companion.default
    get() = newGatewaysDefault()

val Gateways.all: List<Gateway>
    get() = gatewaysGetAllElements(gateways = this)

@Throws(SargonException::class)
fun Gateways.changeCurrent(newCurrent: Gateway) = newGatewaysChangingCurrent(
    to = newCurrent,
    gateways = this
)