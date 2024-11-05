package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.Address
import com.radixdlt.sargon.AddressFormat
import com.radixdlt.sargon.NetworkId
import com.radixdlt.sargon.ResourceAddress
import com.radixdlt.sargon.ResourceAddressKind
import com.radixdlt.sargon.newResourceAddress
import com.radixdlt.sargon.xrdAddressOfNetwork

@Throws(SargonException::class)
fun ResourceAddress.Companion.init(validatingAddress: String) =
    newResourceAddress(bech32 = validatingAddress)

fun ResourceAddress.Companion.xrd(networkId: NetworkId) =
    xrdAddressOfNetwork(networkId = networkId)

val ResourceAddress.string: String
    get() = this.address

val ResourceAddress.isFungible: Boolean
    get() = this.kind == ResourceAddressKind.FUNGIBLE

val ResourceAddress.isNonFungible: Boolean
    get() = this.kind == ResourceAddressKind.FUNGIBLE

val ResourceAddress.isXRD: Boolean
    get() = this == ResourceAddress.xrd(networkId = networkId)

fun ResourceAddress.formatted(
    format: AddressFormat = AddressFormat.DEFAULT
): String = this.formatted.getString(format)

fun ResourceAddress.asGeneral(): Address.Resource = Address.Resource(this)