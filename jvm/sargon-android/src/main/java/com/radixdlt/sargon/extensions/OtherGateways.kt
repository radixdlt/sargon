package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.Gateway
import com.radixdlt.sargon.OtherGateways
import com.radixdlt.sargon.Url
import com.radixdlt.sargon.newOtherGateways
import com.radixdlt.sargon.newOtherGatewaysByAppending
import com.radixdlt.sargon.newOtherGatewaysRemovedById
import com.radixdlt.sargon.newOtherGatewaysRemovedElement
import com.radixdlt.sargon.otherGatewaysElementCount
import com.radixdlt.sargon.otherGatewaysGetElements
import com.radixdlt.sargon.otherGatewaysGetGatewayById

fun OtherGateways.Companion.init(vararg gateway: Gateway): OtherGateways =
    init(gateway.asList())

fun OtherGateways.Companion.init(gateways: List<Gateway>): OtherGateways =
    newOtherGateways(otherGateways = gateways)

operator fun OtherGateways.invoke() = otherGatewaysGetElements(otherGateways = this)

operator fun OtherGateways.get(index: Int) = invoke().get(index = index)

operator fun OtherGateways.contains(element: Gateway) = invoke().contains(element = element)

val OtherGateways.size: Int
    get() = otherGatewaysElementCount(otherGateways = this).toInt()

fun OtherGateways.append(gateway: Gateway): OtherGateways =
    newOtherGatewaysByAppending(gateway = gateway, to = this)

fun OtherGateways.removeByUrl(url: Url): OtherGateways =
    newOtherGatewaysRemovedById(idOfGateway = url, from = this)

fun OtherGateways.remove(gateway: Gateway): OtherGateways =
    newOtherGatewaysRemovedElement(gateway = gateway, from = this)

fun OtherGateways.getBy(url: Url): Gateway? =
    otherGatewaysGetGatewayById(otherGateways = this, id = url)
