package com.radixdlt.sargon.samples

import com.radixdlt.sargon.annotation.UsesSampleValues
import com.radixdlt.sargon.NetworkId
import com.radixdlt.sargon.ResourceAddress
import com.radixdlt.sargon.ValidatorAddress
import com.radixdlt.sargon.newResourceAddressRandom
import com.radixdlt.sargon.newValidatorAddressSampleMainnet
import com.radixdlt.sargon.newValidatorAddressSampleMainnetOther
import com.radixdlt.sargon.newValidatorAddressRandom
import com.radixdlt.sargon.newValidatorAddressSampleStokenet
import com.radixdlt.sargon.newValidatorAddressSampleStokenetOther

@UsesSampleValues
object ValidatorAddressSampleMainnet: SampleWithRandomValues<ValidatorAddress> {
    override fun invoke(): ValidatorAddress = newValidatorAddressSampleMainnet()

    override fun other(): ValidatorAddress = newValidatorAddressSampleMainnetOther()

    override fun random(): ValidatorAddress = newValidatorAddressRandom(
        networkId = NetworkId.MAINNET
    )
}

@UsesSampleValues
val ValidatorAddress.Companion.sampleMainnet: ValidatorAddressSampleMainnet
    get() = ValidatorAddressSampleMainnet

@UsesSampleValues
object ValidatorAddressSampleStokenet: SampleWithRandomValues<ValidatorAddress> {
    override fun invoke(): ValidatorAddress = newValidatorAddressSampleStokenet()

    override fun other(): ValidatorAddress = newValidatorAddressSampleStokenetOther()

    override fun random(): ValidatorAddress = newValidatorAddressRandom(
        networkId = NetworkId.STOKENET
    )
}

@UsesSampleValues
val ValidatorAddress.Companion.sampleStokenet: ValidatorAddressSampleStokenet
    get() = ValidatorAddressSampleStokenet

@UsesSampleValues
fun ValidatorAddress.Companion.sampleRandom(
    networkId: NetworkId
) = newValidatorAddressRandom(networkId = networkId)