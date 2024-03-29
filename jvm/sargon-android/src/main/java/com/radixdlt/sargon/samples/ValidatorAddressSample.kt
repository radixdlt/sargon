package com.radixdlt.sargon.samples

import androidx.annotation.VisibleForTesting
import com.radixdlt.sargon.NetworkId
import com.radixdlt.sargon.ValidatorAddress
import com.radixdlt.sargon.newValidatorAddressSampleMainnet
import com.radixdlt.sargon.newValidatorAddressSampleMainnetOther
import com.radixdlt.sargon.newValidatorAddressSampleRandom
import com.radixdlt.sargon.newValidatorAddressSampleStokenet
import com.radixdlt.sargon.newValidatorAddressSampleStokenetOther

@VisibleForTesting
object ValidatorAddressSampleMainnet: SampleWithRandomValues<ValidatorAddress> {
    override fun invoke(): ValidatorAddress = newValidatorAddressSampleMainnet()

    override fun other(): ValidatorAddress = newValidatorAddressSampleMainnetOther()

    override fun random(): ValidatorAddress = newValidatorAddressSampleRandom(
        networkId = NetworkId.MAINNET
    )
}

@VisibleForTesting
val ValidatorAddress.Companion.sampleMainnet: ValidatorAddressSampleMainnet
    get() = ValidatorAddressSampleMainnet

@VisibleForTesting
object ValidatorAddressSampleStokenet: SampleWithRandomValues<ValidatorAddress> {
    override fun invoke(): ValidatorAddress = newValidatorAddressSampleStokenet()

    override fun other(): ValidatorAddress = newValidatorAddressSampleStokenetOther()

    override fun random(): ValidatorAddress = newValidatorAddressSampleRandom(
        networkId = NetworkId.STOKENET
    )
}

@VisibleForTesting
val ValidatorAddress.Companion.sampleStokenet: ValidatorAddressSampleStokenet
    get() = ValidatorAddressSampleStokenet