package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.Address
import com.radixdlt.sargon.AddressFormat
import com.radixdlt.sargon.NetworkId
import com.radixdlt.sargon.ValidatorAddress
import com.radixdlt.sargon.newValidatorAddress

@Throws(SargonException::class)
fun ValidatorAddress.Companion.init(validatingAddress: String) =
    newValidatorAddress(bech32 = validatingAddress)

val ValidatorAddress.string: String
    get() = this.address

fun ValidatorAddress.formatted(
    format: AddressFormat = AddressFormat.DEFAULT
): String = this.formatted.getString(format)

fun ValidatorAddress.asGeneral(): Address.Validator = Address.Validator(this)