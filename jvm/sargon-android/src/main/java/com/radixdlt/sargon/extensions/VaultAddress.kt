package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.Address
import com.radixdlt.sargon.AddressFormat
import com.radixdlt.sargon.NetworkId
import com.radixdlt.sargon.VaultAddress
import com.radixdlt.sargon.newVaultAddress
import com.radixdlt.sargon.vaultAddressBech32Address
import com.radixdlt.sargon.vaultAddressFormatted
import com.radixdlt.sargon.vaultAddressIsFungible
import com.radixdlt.sargon.vaultAddressIsNonFungible
import com.radixdlt.sargon.vaultAddressNetworkId

@Throws(SargonException::class)
fun VaultAddress.Companion.init(validatingAddress: String) =
    newVaultAddress(bech32 = validatingAddress)

val VaultAddress.string: String
    get() = vaultAddressBech32Address(address = this)

val VaultAddress.networkId: NetworkId
    get() = vaultAddressNetworkId(address = this)

val VaultAddress.isFungible: Boolean
    get() = vaultAddressIsFungible(address = this)

val VaultAddress.isNonFungible: Boolean
    get() = vaultAddressIsNonFungible(address = this)

fun VaultAddress.formatted(
    format: AddressFormat = AddressFormat.DEFAULT
): String = vaultAddressFormatted(address = this, format = format)

fun VaultAddress.asGeneral(): Address.Vault = Address.Vault(this)