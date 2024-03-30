package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.AddressFormat
import com.radixdlt.sargon.AddressOfAccountOrPersona
import com.radixdlt.sargon.NetworkId
import com.radixdlt.sargon.addressOfAccountOrPersonaFormatted
import com.radixdlt.sargon.addressOfAccountOrPersonaNetworkId
import com.radixdlt.sargon.addressOfAccountOrPersonaToString
import com.radixdlt.sargon.newAddressOfAccountOrPersonaFromBech32

@Throws(SargonException::class)
fun AddressOfAccountOrPersona.Companion.init(validating: String) =
    newAddressOfAccountOrPersonaFromBech32(string = validating)

val AddressOfAccountOrPersona.string: String
    get() = addressOfAccountOrPersonaToString(address = this)

val AddressOfAccountOrPersona.networkId: NetworkId
    get() = addressOfAccountOrPersonaNetworkId(address = this)

fun AddressOfAccountOrPersona.formatted(
    format: AddressFormat = AddressFormat.DEFAULT
): String = addressOfAccountOrPersonaFormatted(address = this, format = format)