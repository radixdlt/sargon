package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.AccountAddress
import com.radixdlt.sargon.AddressFormat
import com.radixdlt.sargon.IdentityAddress
import com.radixdlt.sargon.LegacyOlympiaAccountAddress
import com.radixdlt.sargon.NetworkId
import com.radixdlt.sargon.PublicKey
import com.radixdlt.sargon.identityAddressFormatted
import com.radixdlt.sargon.legacyOlympiaAccountAddressFormatted
import com.radixdlt.sargon.legacyOlympiaAccountAddressIsLegacyOfBabylon
import com.radixdlt.sargon.legacyOlympiaAccountAddressToBabylonAccountAddress
import com.radixdlt.sargon.legacyOlympiaAccountAddressToString
import com.radixdlt.sargon.newLegacyOlympiaAccountAddressFromPublicKey
import com.radixdlt.sargon.newLegacyOlympiaAccountAddressFromString

@Throws(SargonException::class)
fun LegacyOlympiaAccountAddress.Companion.init(validatingAddress: String) =
    newLegacyOlympiaAccountAddressFromString(string = validatingAddress)

fun LegacyOlympiaAccountAddress.Companion.init(publicKey: PublicKey.Secp256k1) =
    newLegacyOlympiaAccountAddressFromPublicKey(publicKey = publicKey.value)

val LegacyOlympiaAccountAddress.string: String
    get() = legacyOlympiaAccountAddressToString(address = this)

val LegacyOlympiaAccountAddress.networkId: NetworkId
    // We do not allow creation of Non-Mainnet Olympia Addresses.
    get() = NetworkId.MAINNET

fun LegacyOlympiaAccountAddress.toBabylonAddress() =
    legacyOlympiaAccountAddressToBabylonAccountAddress(address = this)

fun LegacyOlympiaAccountAddress.isLegacyOfBabylonAddress(babylon: AccountAddress) =
    legacyOlympiaAccountAddressIsLegacyOfBabylon(
        legacyOlympiaAddress = this,
        babylonAccountAddress = babylon
    )

fun LegacyOlympiaAccountAddress.formatted(
    format: AddressFormat = AddressFormat.DEFAULT
): String = legacyOlympiaAccountAddressFormatted(address = this, format = format)