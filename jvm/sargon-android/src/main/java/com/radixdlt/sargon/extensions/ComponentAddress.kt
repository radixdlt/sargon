package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.Address
import com.radixdlt.sargon.AddressFormat
import com.radixdlt.sargon.ComponentAddress
import com.radixdlt.sargon.NetworkId
import com.radixdlt.sargon.componentAddressBech32Address
import com.radixdlt.sargon.componentAddressFormatted
import com.radixdlt.sargon.componentAddressIsGlobal
import com.radixdlt.sargon.componentAddressIsInternal
import com.radixdlt.sargon.componentAddressNetworkId
import com.radixdlt.sargon.newComponentAddress

@Throws(SargonException::class)
fun ComponentAddress.Companion.init(validatingAddress: String) =
    newComponentAddress(bech32 = validatingAddress)

val ComponentAddress.string: String
    get() = componentAddressBech32Address(address = this)

val ComponentAddress.networkId: NetworkId
    get() = componentAddressNetworkId(address = this)

/**
 * If the `EntityType == GlobalGenericComponent`
 */
val ComponentAddress.isGlobal: Boolean
    get() = componentAddressIsGlobal(address = this)

/**
 * If the `EntityType == InternalGenericComponent`
 */
val ComponentAddress.isInternal: Boolean
    get() = componentAddressIsInternal(address = this)

fun ComponentAddress.formatted(
    format: AddressFormat = AddressFormat.DEFAULT
): String = componentAddressFormatted(address = this, format = format)

fun ComponentAddress.asGeneral(): Address.Component = Address.Component(this)