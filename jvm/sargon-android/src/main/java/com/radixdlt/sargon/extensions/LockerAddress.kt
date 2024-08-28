package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.Address
import com.radixdlt.sargon.AddressFormat
import com.radixdlt.sargon.LockerAddress
import com.radixdlt.sargon.NetworkId
import com.radixdlt.sargon.lockerAddressBech32Address
import com.radixdlt.sargon.lockerAddressFormatted
import com.radixdlt.sargon.lockerAddressNetworkId
import com.radixdlt.sargon.newLockerAddress

@Throws(SargonException::class)
fun LockerAddress.Companion.init(validatingAddress: String) =
    newLockerAddress(bech32 = validatingAddress)

val LockerAddress.string: String
    get() = lockerAddressBech32Address(address = this)

val LockerAddress.networkId: NetworkId
    get() = lockerAddressNetworkId(address = this)

fun LockerAddress.formatted(
    format: AddressFormat = AddressFormat.DEFAULT
): String = lockerAddressFormatted(address = this, format = format)

fun LockerAddress.asGeneral(): Address.Locker = Address.Locker(this)