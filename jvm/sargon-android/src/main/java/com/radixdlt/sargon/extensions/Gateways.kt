package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.Gateway
import com.radixdlt.sargon.Gateways
import com.radixdlt.sargon.newGateways

fun Gateways.Companion.init(current: Gateway) = newGateways(current = current)