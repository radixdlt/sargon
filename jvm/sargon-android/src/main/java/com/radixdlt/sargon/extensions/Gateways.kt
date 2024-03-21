package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.Gateway
import com.radixdlt.sargon.Gateways
import com.radixdlt.sargon.newGateways

/**
 * Constructs [Gateways] with [current] set as active Gateway.
 */
fun Gateways.Companion.init(current: Gateway) = newGateways(current = current)