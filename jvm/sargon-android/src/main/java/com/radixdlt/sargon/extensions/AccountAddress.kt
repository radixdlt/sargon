package com.radixdlt.sargon.extensions

// import com.radixdlt.sargon.accountAddressFormatted
import com.radixdlt.sargon.AccountAddress
import com.radixdlt.sargon.AddressFormat
import com.radixdlt.sargon.LegacyOlympiaAccountAddress
import com.radixdlt.sargon.NetworkId
import com.radixdlt.sargon.PublicKey
import com.radixdlt.sargon.accountAddressBech32Address
import com.radixdlt.sargon.accountAddressFormatted
import com.radixdlt.sargon.accountAddressIsLegacy
import com.radixdlt.sargon.accountAddressNetworkId
import com.radixdlt.sargon.newAccountAddress
import com.radixdlt.sargon.newAccountAddressFrom

@Throws(SargonException::class)
fun AccountAddress.Companion.init(validatingAddress: String) =
    newAccountAddress(bech32 = validatingAddress)

fun AccountAddress.Companion.init(publicKey: PublicKey, networkId: NetworkId) =
    newAccountAddressFrom(publicKey = publicKey, networkId = networkId)

val AccountAddress.string: String
    get() = accountAddressBech32Address(address = this)

val AccountAddress.networkId: NetworkId
    get() = accountAddressNetworkId(address = this)

/**
 * Returns `false` for all addresses created with [com.radixdlt.sargon.Ed25519PublicKey], i.e.
 * for all accounts created by the Babylon Radix Wallets.
 *
 * Returns `true` for all addresses created with [com.radixdlt.sargon.Secp256k1PublicKey], i.e.
 * imported from the Olympia Wallet.
 */
val AccountAddress.isLegacy
    get() = accountAddressIsLegacy(address = this)

fun AccountAddress.formatted(
    format: AddressFormat = AddressFormat.DEFAULT
): String = accountAddressFormatted(address = this, format = format)

fun AccountAddress.wasMigratedFromLegacyOlympia(legacy: LegacyOlympiaAccountAddress) =
    legacy.isLegacyOfBabylonAddress(babylon = this)