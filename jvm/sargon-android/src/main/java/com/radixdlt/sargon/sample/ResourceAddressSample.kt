package com.radixdlt.sargon.sample

import androidx.annotation.VisibleForTesting
import androidx.compose.ui.tooling.preview.PreviewParameterProvider
import com.radixdlt.sargon.ResourceAddress
import com.radixdlt.sargon.newResourceAddressSampleMainnetCandy
import com.radixdlt.sargon.newResourceAddressSampleMainnetNftGcMembership
import com.radixdlt.sargon.newResourceAddressSampleMainnetXrd
import com.radixdlt.sargon.newResourceAddressSampleStokenetCandy
import com.radixdlt.sargon.newResourceAddressSampleStokenetGcTokens
import com.radixdlt.sargon.newResourceAddressSampleStokenetGum
import com.radixdlt.sargon.newResourceAddressSampleStokenetXrd

@VisibleForTesting
val ResourceAddress.Companion.sampleMainnet: Sample<ResourceAddress>
    get() = object : Sample<ResourceAddress> {
        override val all: List<ResourceAddress>
            get() = listOf(
                mainnetXrd,
                mainnetCandy,
                mainnetNonFungibleGCMembership
            )

        override fun invoke(): ResourceAddress = mainnetXrd

        override fun other(): ResourceAddress = mainnetCandy

        val mainnetXrd: ResourceAddress
            get() = newResourceAddressSampleMainnetXrd()

        val mainnetCandy: ResourceAddress
            get() = newResourceAddressSampleMainnetCandy()

        val mainnetNonFungibleGCMembership: ResourceAddress
            get() = newResourceAddressSampleMainnetNftGcMembership()
    }

@VisibleForTesting
val ResourceAddress.Companion.sampleStokenet: Sample<ResourceAddress>
    get() = object : Sample<ResourceAddress> {
        override val all: List<ResourceAddress>
            get() = listOf(
                stokenetXRD,
                stokenetGC,
                stokenetGum,
                stokenetCandy
            )

        override fun invoke(): ResourceAddress = stokenetXRD

        override fun other(): ResourceAddress = stokenetCandy

        val stokenetXRD: ResourceAddress
            get() = newResourceAddressSampleStokenetXrd()

        val stokenetGum: ResourceAddress
            get() = newResourceAddressSampleStokenetGum()

        val stokenetGC: ResourceAddress
            get() = newResourceAddressSampleStokenetGcTokens()

        val stokenetCandy: ResourceAddress
            get() = newResourceAddressSampleStokenetCandy()
    }

class ResourceAddressMainnetPreviewParameterProvider: PreviewParameterProvider<ResourceAddress> {
    override val values: Sequence<ResourceAddress>
        get() = ResourceAddress.sampleMainnet.all.asSequence()

}

class ResourceAddressStokenetPreviewParameterProvider: PreviewParameterProvider<ResourceAddress> {
    override val values: Sequence<ResourceAddress>
        get() = ResourceAddress.sampleStokenet.all.asSequence()

}