@file:JvmName("GatewaysKt")

package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.Gateway
import com.radixdlt.sargon.Gateways
import com.radixdlt.sargon.Url
import com.radixdlt.sargon.gatewaysElementCount
import com.radixdlt.sargon.gatewaysGetElements
import com.radixdlt.sargon.gatewaysGetGatewayById
import com.radixdlt.sargon.newGateways
import com.radixdlt.sargon.newGatewaysByAppending
import com.radixdlt.sargon.newGatewaysByUpdatingOrAppending
import com.radixdlt.sargon.newGatewaysByUpdatingOrInsertingAtIndex
import com.radixdlt.sargon.newGatewaysRemovedById
import com.radixdlt.sargon.newGatewaysRemovedElement

fun Gateways.Companion.init(vararg gateway: Gateway): Gateways =
    init(gateway.asList())

fun Gateways.Companion.init(gateways: List<Gateway>): Gateways =
    newGateways(gateways = gateways)

operator fun Gateways.invoke() = gatewaysGetElements(gateways = this)

operator fun Gateways.get(index: Int) = invoke().get(index = index)

operator fun Gateways.contains(element: Gateway) = invoke().contains(element = element)

val Gateways.size: Int
    get() = gatewaysElementCount(gateways = this).toInt()

fun Gateways.append(gateway: Gateway): Gateways =
    newGatewaysByAppending(gateway = gateway, to = this)

fun Gateways.updateOrInsert(gateway: Gateway, index: Int): Gateways =
    newGatewaysByUpdatingOrInsertingAtIndex(
        gateway = gateway,
        to = this,
        index = index.toULong()
    )

fun Gateways.updateOrAppend(gateway: Gateway): Gateways =
    newGatewaysByUpdatingOrAppending(gateway = gateway, to = this)

fun Gateways.removeByUrl(url: Url): Gateways =
    newGatewaysRemovedById(idOfGateway = url, from = this)

fun Gateways.remove(gateway: Gateway): Gateways =
    newGatewaysRemovedElement(gateway = gateway, from = this)

fun Gateways.getBy(url: Url): Gateway? = gatewaysGetGatewayById(gateways = this, id = url)
