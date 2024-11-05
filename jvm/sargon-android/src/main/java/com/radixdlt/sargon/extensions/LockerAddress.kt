package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.Address
import com.radixdlt.sargon.AddressFormat
import com.radixdlt.sargon.LockerAddress
import com.radixdlt.sargon.NetworkId
import com.radixdlt.sargon.newLockerAddress

@Throws(SargonException::class)
fun LockerAddress.Companion.init(validatingAddress: String) =
    newLockerAddress(bech32 = validatingAddress)

val LockerAddress.string: String
    get() = this.address

fun LockerAddress.formatted(
    format: AddressFormat = AddressFormat.DEFAULT
): String = this.formatted.getString(format)

fun LockerAddress.asGeneral(): Address.Locker = Address.Locker(this)