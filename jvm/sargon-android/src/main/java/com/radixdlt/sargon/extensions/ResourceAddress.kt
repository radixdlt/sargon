package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.NetworkId
import com.radixdlt.sargon.ResourceAddress
import com.radixdlt.sargon.newResourceAddress
import com.radixdlt.sargon.resourceAddressBech32Address
import com.radixdlt.sargon.resourceAddressIsFungible
import com.radixdlt.sargon.resourceAddressIsNonFungible
import com.radixdlt.sargon.resourceAddressNetworkId

fun ResourceAddress.Companion.init(validatingAddress: String) =
    newResourceAddress(bech32 = validatingAddress)

val ResourceAddress.string: String
    get() = resourceAddressBech32Address(address = this)

val ResourceAddress.networkId: NetworkId
    get() = resourceAddressNetworkId(address = this)

val ResourceAddress.isFungible: Boolean
    get() = resourceAddressIsFungible(address = this)

val ResourceAddress.isNonFungible: Boolean
    get() = resourceAddressIsNonFungible(address = this)