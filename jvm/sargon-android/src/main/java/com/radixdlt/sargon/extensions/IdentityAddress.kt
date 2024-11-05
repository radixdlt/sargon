package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.Address
import com.radixdlt.sargon.AddressFormat
import com.radixdlt.sargon.IdentityAddress
import com.radixdlt.sargon.NetworkId
import com.radixdlt.sargon.PublicKey
import com.radixdlt.sargon.newIdentityAddress
import com.radixdlt.sargon.newIdentityAddressFrom

@Throws(SargonException::class)
fun IdentityAddress.Companion.init(validatingAddress: String) =
    newIdentityAddress(bech32 = validatingAddress)

fun IdentityAddress.Companion.init(publicKey: PublicKey, networkId: NetworkId) =
    newIdentityAddressFrom(publicKey = publicKey, networkId = networkId)

val IdentityAddress.string: String
    get() = this.address

fun IdentityAddress.formatted(
    format: AddressFormat = AddressFormat.DEFAULT
): String = this.formatted.getString(format)

fun IdentityAddress.asGeneral(): Address.Identity = Address.Identity(this)