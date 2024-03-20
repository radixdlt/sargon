package com.radixdlt.sargon.samples

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
object ResourceAddressSampleMainnet: Sample<ResourceAddress> {
    override val all: List<ResourceAddress>
        get() = listOf(
            xrd,
            candy,
            nonFungibleGCMembership
        )

    override fun invoke(): ResourceAddress = xrd

    override fun other(): ResourceAddress = candy

    val xrd: ResourceAddress
        get() = newResourceAddressSampleMainnetXrd()

    val candy: ResourceAddress
        get() = newResourceAddressSampleMainnetCandy()

    val nonFungibleGCMembership: ResourceAddress
        get() = newResourceAddressSampleMainnetNftGcMembership()
}

@VisibleForTesting
object ResourceAddressSampleStokenet: Sample<ResourceAddress> {
    override val all: List<ResourceAddress>
        get() = listOf(
            xrd,
            gc,
            gum,
            candy
        )

    override fun invoke(): ResourceAddress = xrd

    override fun other(): ResourceAddress = candy

    val xrd: ResourceAddress
        get() = newResourceAddressSampleStokenetXrd()

    val gum: ResourceAddress
        get() = newResourceAddressSampleStokenetGum()

    val gc: ResourceAddress
        get() = newResourceAddressSampleStokenetGcTokens()

    val candy: ResourceAddress
        get() = newResourceAddressSampleStokenetCandy()
}

@VisibleForTesting
val ResourceAddress.Companion.sampleMainnet: ResourceAddressSampleMainnet
    get() = ResourceAddressSampleMainnet

@VisibleForTesting
val ResourceAddress.Companion.sampleStokenet: ResourceAddressSampleStokenet
    get() = ResourceAddressSampleStokenet

class ResourceAddressMainnetPreviewParameterProvider: PreviewParameterProvider<ResourceAddress> {
    override val values: Sequence<ResourceAddress>
        get() = ResourceAddress.sampleMainnet.all.asSequence()

}

class ResourceAddressStokenetPreviewParameterProvider: PreviewParameterProvider<ResourceAddress> {
    override val values: Sequence<ResourceAddress>
        get() = ResourceAddress.sampleStokenet.all.asSequence()

}