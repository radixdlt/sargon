package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.NetworkId
import com.radixdlt.sargon.NonFungibleResourceAddress
import com.radixdlt.sargon.newNonFungibleResourceAddress
import com.radixdlt.sargon.nonFungibleResourceAddressBech32Address
import com.radixdlt.sargon.nonFungibleResourceAddressNetworkId

fun NonFungibleResourceAddress.Companion.init(validating: String) =
    newNonFungibleResourceAddress(bech32 = validating)

val NonFungibleResourceAddress.string: String
    get() = nonFungibleResourceAddressBech32Address(address = this)

val NonFungibleResourceAddress.networkId: NetworkId
    get() = nonFungibleResourceAddressNetworkId(address = this)