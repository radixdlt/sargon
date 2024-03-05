package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.NetworkId
import com.radixdlt.sargon.PackageAddress
import com.radixdlt.sargon.newPackageAddress
import com.radixdlt.sargon.packageAddressBech32Address
import com.radixdlt.sargon.packageAddressNetworkId

fun PackageAddress.Companion.init(validatingAddress: String) =
    newPackageAddress(bech32 = validatingAddress)

val PackageAddress.address: String
    get() = packageAddressBech32Address(address = this)

val PackageAddress.networkId: NetworkId
    get() = packageAddressNetworkId(address = this)