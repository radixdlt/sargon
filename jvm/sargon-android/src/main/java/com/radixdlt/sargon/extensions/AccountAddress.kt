package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.AccountAddress
import com.radixdlt.sargon.Address
import com.radixdlt.sargon.AddressFormat
import com.radixdlt.sargon.LegacyOlympiaAccountAddress
import com.radixdlt.sargon.NetworkId
import com.radixdlt.sargon.PublicKey
import com.radixdlt.sargon.newAccountAddress
import com.radixdlt.sargon.newAccountAddressFrom

@Throws(SargonException::class)
fun AccountAddress.Companion.init(validatingAddress: String) =
    newAccountAddress(bech32 = validatingAddress)

fun AccountAddress.Companion.init(publicKey: PublicKey, networkId: NetworkId) =
    newAccountAddressFrom(publicKey = publicKey, networkId = networkId)

val AccountAddress.string: String
    get() = this.address

fun AccountAddress.formatted(
    format: AddressFormat = AddressFormat.DEFAULT
): String = this.formatted.getString(format)

fun AccountAddress.wasMigratedFromLegacyOlympia(legacy: LegacyOlympiaAccountAddress) =
    legacy.isLegacyOfBabylonAddress(babylon = this)

fun AccountAddress.asGeneral(): Address.Account = Address.Account(this)