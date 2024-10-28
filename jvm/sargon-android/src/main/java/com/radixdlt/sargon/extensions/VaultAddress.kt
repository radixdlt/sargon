package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.Address
import com.radixdlt.sargon.AddressFormat
import com.radixdlt.sargon.NetworkId
import com.radixdlt.sargon.VaultAddress
import com.radixdlt.sargon.newVaultAddress
import com.radixdlt.sargon.vaultAddressIsFungible
import com.radixdlt.sargon.vaultAddressIsNonFungible

@Throws(SargonException::class)
fun VaultAddress.Companion.init(validatingAddress: String) =
    newVaultAddress(bech32 = validatingAddress)

val VaultAddress.string: String
    get() = this.address

val VaultAddress.isFungible: Boolean
    get() = vaultAddressIsFungible(address = this)

val VaultAddress.isNonFungible: Boolean
    get() = vaultAddressIsNonFungible(address = this)

fun VaultAddress.formatted(
    format: AddressFormat = AddressFormat.DEFAULT
): String = this.formatted.getString(format)

fun VaultAddress.asGeneral(): Address.Vault = Address.Vault(this)