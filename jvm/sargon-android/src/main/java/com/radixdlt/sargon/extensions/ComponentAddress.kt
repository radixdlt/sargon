package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.ComponentAddress
import com.radixdlt.sargon.NetworkId
import com.radixdlt.sargon.componentAddressBech32Address
import com.radixdlt.sargon.componentAddressNetworkId
import com.radixdlt.sargon.newComponentAddress

@Throws(SargonException::class)
fun ComponentAddress.Companion.init(validatingAddress: String) =
    newComponentAddress(bech32 = validatingAddress)

val ComponentAddress.string: String
    get() = componentAddressBech32Address(address = this)

val ComponentAddress.networkId: NetworkId
    get() = componentAddressNetworkId(address = this)