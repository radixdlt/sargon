package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.AddressFormat
import com.radixdlt.sargon.IdentityAddress
import com.radixdlt.sargon.NetworkId
import com.radixdlt.sargon.PackageAddress
import com.radixdlt.sargon.identityAddressFormatted
import com.radixdlt.sargon.newPackageAddress
import com.radixdlt.sargon.packageAddressBech32Address
import com.radixdlt.sargon.packageAddressFormatted
import com.radixdlt.sargon.packageAddressNetworkId

@Throws(SargonException::class)
fun PackageAddress.Companion.init(validatingAddress: String) =
    newPackageAddress(bech32 = validatingAddress)

val PackageAddress.string: String
    get() = packageAddressBech32Address(address = this)

val PackageAddress.networkId: NetworkId
    get() = packageAddressNetworkId(address = this)

fun PackageAddress.formatted(
    format: AddressFormat = AddressFormat.DEFAULT
): String = packageAddressFormatted(address = this, format = format)