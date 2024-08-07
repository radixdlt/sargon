package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.AccessControllerAddress
import com.radixdlt.sargon.Address
import com.radixdlt.sargon.AddressFormat
import com.radixdlt.sargon.NetworkId
import com.radixdlt.sargon.accessControllerAddressBech32Address
import com.radixdlt.sargon.accessControllerAddressFormatted
import com.radixdlt.sargon.accessControllerAddressNetworkId
import com.radixdlt.sargon.newAccessControllerAddress

@Throws(SargonException::class)
fun AccessControllerAddress.Companion.init(validatingAddress: String) =
    newAccessControllerAddress(bech32 = validatingAddress)

val AccessControllerAddress.string: String
    get() = accessControllerAddressBech32Address(address = this)

val AccessControllerAddress.networkId: NetworkId
    get() = accessControllerAddressNetworkId(address = this)

fun AccessControllerAddress.formatted(
    format: AddressFormat = AddressFormat.DEFAULT
): String = accessControllerAddressFormatted(address = this, format = format)

fun AccessControllerAddress.asGeneral(): Address.AccessController
    = Address.AccessController(this)