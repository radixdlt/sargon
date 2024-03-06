package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.NetworkId
import com.radixdlt.sargon.ValidatorAddress
import com.radixdlt.sargon.newValidatorAddress
import com.radixdlt.sargon.validatorAddressBech32Address
import com.radixdlt.sargon.validatorAddressNetworkId

fun ValidatorAddress.Companion.init(validatingAddress: String) =
    newValidatorAddress(bech32 = validatingAddress)

val ValidatorAddress.string: String
    get() = validatorAddressBech32Address(address = this)

val ValidatorAddress.networkId: NetworkId
    get() = validatorAddressNetworkId(address = this)