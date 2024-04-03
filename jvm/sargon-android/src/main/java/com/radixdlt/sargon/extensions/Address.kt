package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.Address
import com.radixdlt.sargon.AddressFormat
import com.radixdlt.sargon.NetworkId
import com.radixdlt.sargon.addressFormatted
import com.radixdlt.sargon.addressNetworkId
import com.radixdlt.sargon.addressToString
import com.radixdlt.sargon.newAddressFromBech32

@Throws(SargonException::class)
fun Address.Companion.init(validatingAddress: String) = newAddressFromBech32(validatingAddress)

fun Address.formatted(format: AddressFormat = AddressFormat.DEFAULT) =
    addressFormatted(address = this, format = format)

val Address.networkId: NetworkId
    get() = addressNetworkId(address = this)

val Address.string: String
    get() = addressToString(address = this)
