package com.radixdlt.sargon.samples

import androidx.annotation.VisibleForTesting
import androidx.compose.ui.tooling.preview.PreviewParameterProvider
import com.radixdlt.sargon.ValidatorAddress
import com.radixdlt.sargon.newValidatorAddressSampleMainnet
import com.radixdlt.sargon.newValidatorAddressSampleMainnetOther
import com.radixdlt.sargon.newValidatorAddressSampleStokenet
import com.radixdlt.sargon.newValidatorAddressSampleStokenetOther

@VisibleForTesting
val ValidatorAddress.Companion.sampleMainnet: Sample<ValidatorAddress>
    get() = object : Sample<ValidatorAddress> {

        override fun invoke(): ValidatorAddress = newValidatorAddressSampleMainnet()

        override fun other(): ValidatorAddress = newValidatorAddressSampleMainnetOther()

        val stokenet: ValidatorAddress
            get() = newValidatorAddressSampleStokenet()

        val stokenetOther: ValidatorAddress
            get() = newValidatorAddressSampleStokenetOther()
    }

@VisibleForTesting
val ValidatorAddress.Companion.sampleStokenet: Sample<ValidatorAddress>
    get() = object : Sample<ValidatorAddress> {

        override fun invoke(): ValidatorAddress = newValidatorAddressSampleStokenet()

        override fun other(): ValidatorAddress = newValidatorAddressSampleStokenetOther()
    }


class ValidatorAddressMainnetPreviewParameterProvider: PreviewParameterProvider<ValidatorAddress> {
    override val values: Sequence<ValidatorAddress>
        get() = ValidatorAddress.sampleMainnet.all.asSequence()

}

class ValidatorAddressStokenetPreviewParameterProvider: PreviewParameterProvider<ValidatorAddress> {
    override val values: Sequence<ValidatorAddress>
        get() = ValidatorAddress.sampleStokenet.all.asSequence()

}