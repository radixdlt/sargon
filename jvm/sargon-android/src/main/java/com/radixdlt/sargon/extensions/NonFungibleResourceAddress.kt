package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.AddressFormat
import com.radixdlt.sargon.IdentityAddress
import com.radixdlt.sargon.NetworkId
import com.radixdlt.sargon.NonFungibleResourceAddress
import com.radixdlt.sargon.ResourceAddress
import com.radixdlt.sargon.identityAddressFormatted
import com.radixdlt.sargon.newNonFungibleResourceAddress
import com.radixdlt.sargon.nonFungibleResourceAddressBech32Address
import com.radixdlt.sargon.nonFungibleResourceAddressNetworkId

/**
 * Tries to bech32 decode the string into a specialized address.
 */
@Throws(SargonException::class)
fun NonFungibleResourceAddress.Companion.init(validating: String) =
    newNonFungibleResourceAddress(bech32 = validating)

/**
 * Returns the bech32 encoding of this address
 */
val NonFungibleResourceAddress.string: String
    get() = nonFungibleResourceAddressBech32Address(address = this)

/**
 * Returns the network id this address
 */
val NonFungibleResourceAddress.networkId: NetworkId
    get() = nonFungibleResourceAddressNetworkId(address = this)

fun NonFungibleResourceAddress.formatted(
    format: AddressFormat = AddressFormat.DEFAULT
): String = ResourceAddress.init(string).formatted(format = format)