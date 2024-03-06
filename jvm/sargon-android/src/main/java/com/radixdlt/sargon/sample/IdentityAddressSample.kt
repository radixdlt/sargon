package com.radixdlt.sargon.sample

import androidx.annotation.VisibleForTesting
import androidx.compose.ui.tooling.preview.PreviewParameterProvider
import com.radixdlt.sargon.IdentityAddress
import com.radixdlt.sargon.newIdentityAddressSampleMainnet
import com.radixdlt.sargon.newIdentityAddressSampleMainnetOther
import com.radixdlt.sargon.newIdentityAddressSampleStokenet
import com.radixdlt.sargon.newIdentityAddressSampleStokenetOther

@VisibleForTesting
val IdentityAddress.Companion.sampleMainnet: Sample<IdentityAddress>
    get() = object : Sample<IdentityAddress> {
        override fun invoke(): IdentityAddress = newIdentityAddressSampleMainnet()

        override fun other(): IdentityAddress = newIdentityAddressSampleMainnetOther()
    }

@VisibleForTesting
val IdentityAddress.Companion.sampleStokenet: Sample<IdentityAddress>
    get() = object : Sample<IdentityAddress> {
        override fun invoke(): IdentityAddress = newIdentityAddressSampleStokenet()

        override fun other(): IdentityAddress = newIdentityAddressSampleStokenetOther()
    }

class IdentityAddressMainnetPreviewParameterProvider: PreviewParameterProvider<IdentityAddress> {
    override val values: Sequence<IdentityAddress>
        get() = IdentityAddress.sampleMainnet.all.asSequence()

}

class IdentityAddressStokenetPreviewParameterProvider: PreviewParameterProvider<IdentityAddress> {
    override val values: Sequence<IdentityAddress>
        get() = IdentityAddress.sampleStokenet.all.asSequence()

}