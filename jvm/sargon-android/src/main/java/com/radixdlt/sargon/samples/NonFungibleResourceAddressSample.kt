package com.radixdlt.sargon.samples

import androidx.annotation.VisibleForTesting
import androidx.compose.ui.tooling.preview.PreviewParameterProvider
import com.radixdlt.sargon.NonFungibleResourceAddress
import com.radixdlt.sargon.newNonFungibleResourceAddressSampleMainnet
import com.radixdlt.sargon.newNonFungibleResourceAddressSampleMainnetOther
import com.radixdlt.sargon.newNonFungibleResourceAddressSampleStokenet
import com.radixdlt.sargon.newNonFungibleResourceAddressSampleStokenetOther

@VisibleForTesting
val NonFungibleResourceAddress.Companion.sampleMainnet: Sample<NonFungibleResourceAddress>
    get() = object : Sample<NonFungibleResourceAddress> {

        override fun invoke(): NonFungibleResourceAddress =
            newNonFungibleResourceAddressSampleMainnet()

        override fun other(): NonFungibleResourceAddress =
            newNonFungibleResourceAddressSampleMainnetOther()

    }

class NonFungibleResourceAddressMainnetPreviewParameterProvider :
    PreviewParameterProvider<NonFungibleResourceAddress> {
    override val values: Sequence<NonFungibleResourceAddress>
        get() = NonFungibleResourceAddress.sampleMainnet.all.asSequence()
}

@VisibleForTesting
val NonFungibleResourceAddress.Companion.sampleStokenet: Sample<NonFungibleResourceAddress>
    get() = object : Sample<NonFungibleResourceAddress> {

        override fun invoke(): NonFungibleResourceAddress =
            newNonFungibleResourceAddressSampleStokenet()

        override fun other(): NonFungibleResourceAddress =
            newNonFungibleResourceAddressSampleStokenetOther()

    }

class NonFungibleResourceAddressStokenetPreviewParameterProvider :
    PreviewParameterProvider<NonFungibleResourceAddress> {
    override val values: Sequence<NonFungibleResourceAddress>
        get() = NonFungibleResourceAddress.sampleStokenet.all.asSequence()
}