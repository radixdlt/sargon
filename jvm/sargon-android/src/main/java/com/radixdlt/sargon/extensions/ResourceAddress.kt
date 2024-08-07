package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.Address
import com.radixdlt.sargon.AddressFormat
import com.radixdlt.sargon.NetworkId
import com.radixdlt.sargon.ResourceAddress
import com.radixdlt.sargon.newResourceAddress
import com.radixdlt.sargon.resourceAddressBech32Address
import com.radixdlt.sargon.resourceAddressFormatted
import com.radixdlt.sargon.resourceAddressIsFungible
import com.radixdlt.sargon.resourceAddressIsNonFungible
import com.radixdlt.sargon.resourceAddressNetworkId
import com.radixdlt.sargon.xrdAddressOfNetwork

@Throws(SargonException::class)
fun ResourceAddress.Companion.init(validatingAddress: String) =
    newResourceAddress(bech32 = validatingAddress)

fun ResourceAddress.Companion.xrd(networkId: NetworkId) =
    xrdAddressOfNetwork(networkId = networkId)

val ResourceAddress.string: String
    get() = resourceAddressBech32Address(address = this)

val ResourceAddress.networkId: NetworkId
    get() = resourceAddressNetworkId(address = this)

val ResourceAddress.isFungible: Boolean
    get() = resourceAddressIsFungible(address = this)

val ResourceAddress.isNonFungible: Boolean
    get() = resourceAddressIsNonFungible(address = this)

val ResourceAddress.isXRD: Boolean
    get() = this == ResourceAddress.xrd(networkId = networkId)

fun ResourceAddress.formatted(
    format: AddressFormat = AddressFormat.DEFAULT
): String = resourceAddressFormatted(address = this, format = format)

fun ResourceAddress.asGeneral(): Address.Resource = Address.Resource(this)