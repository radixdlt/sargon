package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.AddressFormat
import com.radixdlt.sargon.ComponentAddress
import com.radixdlt.sargon.IdentityAddress
import com.radixdlt.sargon.NetworkId
import com.radixdlt.sargon.PublicKey
import com.radixdlt.sargon.componentAddressFormatted
import com.radixdlt.sargon.identityAddressBech32Address
import com.radixdlt.sargon.identityAddressFormatted
import com.radixdlt.sargon.identityAddressNetworkId
import com.radixdlt.sargon.newIdentityAddress
import com.radixdlt.sargon.newIdentityAddressFrom

@Throws(SargonException::class)
fun IdentityAddress.Companion.init(validatingAddress: String) =
    newIdentityAddress(bech32 = validatingAddress)

fun IdentityAddress.Companion.init(publicKey: PublicKey, networkId: NetworkId) =
    newIdentityAddressFrom(publicKey = publicKey, networkId = networkId)

val IdentityAddress.string: String
    get() = identityAddressBech32Address(address = this)

val IdentityAddress.networkId: NetworkId
    get() = identityAddressNetworkId(address = this)

fun IdentityAddress.formatted(
    format: AddressFormat = AddressFormat.DEFAULT
): String = identityAddressFormatted(address = this, format = format)