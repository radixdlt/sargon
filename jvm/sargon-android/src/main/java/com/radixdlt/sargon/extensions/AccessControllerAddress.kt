package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.AccessControllerAddress
import com.radixdlt.sargon.Address
import com.radixdlt.sargon.AddressFormat
import com.radixdlt.sargon.FormattedAddress
import com.radixdlt.sargon.newAccessControllerAddress

@Throws(SargonException::class)
fun AccessControllerAddress.Companion.init(validatingAddress: String) =
    newAccessControllerAddress(bech32 = validatingAddress)

val AccessControllerAddress.string: String
    get() = this.address

fun AccessControllerAddress.formatted(
    format: AddressFormat = AddressFormat.DEFAULT
): String = this.formatted.getString(format)

fun AccessControllerAddress.asGeneral(): Address.AccessController
    = Address.AccessController(this)

fun FormattedAddress.getString(
    format: AddressFormat
): String = when (format) {
    AddressFormat.DEFAULT -> this.default
    AddressFormat.RAW -> this.raw
    AddressFormat.FULL -> this.full
}