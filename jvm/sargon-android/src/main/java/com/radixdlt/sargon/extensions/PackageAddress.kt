package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.Address
import com.radixdlt.sargon.AddressFormat
import com.radixdlt.sargon.NetworkId
import com.radixdlt.sargon.PackageAddress
import com.radixdlt.sargon.newPackageAddress

@Throws(SargonException::class)
fun PackageAddress.Companion.init(validatingAddress: String) =
    newPackageAddress(bech32 = validatingAddress)

val PackageAddress.string: String
    get() = this.address

fun PackageAddress.formatted(
    format: AddressFormat = AddressFormat.DEFAULT
): String = this.formatted.getString(format)

fun PackageAddress.asGeneral(): Address.Package = Address.Package(this)