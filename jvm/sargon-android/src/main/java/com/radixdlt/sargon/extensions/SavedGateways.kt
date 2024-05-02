package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.Gateway
import com.radixdlt.sargon.Gateways
import com.radixdlt.sargon.SavedGateways
import com.radixdlt.sargon.newSavedGateways
import com.radixdlt.sargon.newSavedGatewaysChangingCurrent
import com.radixdlt.sargon.newSavedGatewaysDefault
import com.radixdlt.sargon.savedGatewaysGetAllElements

/**
 * Constructs [Gateways] with [current] set as active Gateway.
 */
fun SavedGateways.Companion.init(current: Gateway) = newSavedGateways(current = current)

/**
 * The default configuration for each new user:
 * Current: mainnet
 * Other: [ stokenet ]
 */
val SavedGateways.Companion.default
    get() = newSavedGatewaysDefault()

val SavedGateways.all: List<Gateway>
    get() = savedGatewaysGetAllElements(gateways = this)

@Throws(SargonException::class)
fun SavedGateways.changeCurrent(newCurrent: Gateway) = newSavedGatewaysChangingCurrent(
    to = newCurrent,
    gateways = this
)