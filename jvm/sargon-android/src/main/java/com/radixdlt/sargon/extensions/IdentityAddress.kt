package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.IdentityAddress
import com.radixdlt.sargon.NetworkId
import com.radixdlt.sargon.PublicKey
import com.radixdlt.sargon.identityAddressBech32Address
import com.radixdlt.sargon.identityAddressNetworkId
import com.radixdlt.sargon.newIdentityAddress
import com.radixdlt.sargon.newIdentityAddressFrom

fun IdentityAddress.Companion.init(validatingAddress: String) =
    newIdentityAddress(bech32 = validatingAddress)

fun IdentityAddress.Companion.init(publicKey: PublicKey, networkId: NetworkId) =
    newIdentityAddressFrom(publicKey = publicKey, networkId = networkId)

val IdentityAddress.string: String
    get() = identityAddressBech32Address(address = this)

val IdentityAddress.networkId: NetworkId
    get() = identityAddressNetworkId(address = this)