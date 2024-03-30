package com.radixdlt.sargon.samples

import androidx.annotation.VisibleForTesting
import com.radixdlt.sargon.NetworkId
import com.radixdlt.sargon.PoolAddress
import com.radixdlt.sargon.ResourceAddress
import com.radixdlt.sargon.newPoolAddressRandom
import com.radixdlt.sargon.newResourceAddressSampleMainnetCandy
import com.radixdlt.sargon.newResourceAddressSampleMainnetNftGcMembership
import com.radixdlt.sargon.newResourceAddressSampleMainnetXrd
import com.radixdlt.sargon.newResourceAddressRandom
import com.radixdlt.sargon.newResourceAddressSampleStokenetCandy
import com.radixdlt.sargon.newResourceAddressSampleStokenetGcTokens
import com.radixdlt.sargon.newResourceAddressSampleStokenetGum
import com.radixdlt.sargon.newResourceAddressSampleStokenetXrd

@VisibleForTesting
object ResourceAddressSampleMainnet: SampleWithRandomValues<ResourceAddress> {
    override val all: List<ResourceAddress>
        get() = listOf(
            xrd,
            candy,
            nonFungibleGCMembership
        )

    override fun invoke(): ResourceAddress = xrd

    override fun other(): ResourceAddress = candy

    override fun random(): ResourceAddress = newResourceAddressRandom(
        networkId = NetworkId.MAINNET
    )

    val xrd: ResourceAddress
        get() = newResourceAddressSampleMainnetXrd()

    val candy: ResourceAddress
        get() = newResourceAddressSampleMainnetCandy()

    val nonFungibleGCMembership: ResourceAddress
        get() = newResourceAddressSampleMainnetNftGcMembership()
}

@VisibleForTesting
object ResourceAddressSampleStokenet: SampleWithRandomValues<ResourceAddress> {
    override val all: List<ResourceAddress>
        get() = listOf(
            xrd,
            gc,
            gum,
            candy
        )

    override fun invoke(): ResourceAddress = xrd

    override fun other(): ResourceAddress = candy

    override fun random(): ResourceAddress = newResourceAddressRandom(
        networkId = NetworkId.STOKENET
    )

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

@VisibleForTesting
fun ResourceAddress.Companion.sampleRandom(
    networkId: NetworkId
) = newResourceAddressRandom(networkId = networkId)