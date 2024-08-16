package com.radixdlt.sargon

import com.radixdlt.sargon.extensions.AssetPreferences
import com.radixdlt.sargon.extensions.hiddenAssets
import com.radixdlt.sargon.extensions.hideAsset
import com.radixdlt.sargon.extensions.unhideAsset
import com.radixdlt.sargon.samples.Sample
import com.radixdlt.sargon.samples.sample
import com.radixdlt.sargon.samples.sampleMainnet
import com.radixdlt.sargon.samples.sampleStokenet
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Assertions.assertTrue
import org.junit.jupiter.api.Test

class AssetPreferencesTest : SampleTestable<AssetPreferences> {

    override val samples: List<Sample<AssetPreferences>>
        get() = listOf(AssetPreferences.sample)

    @Test
    fun testHiddenAssets() {
        var sut = AssetPreferences()

        assertTrue(sut.hiddenAssets.isEmpty())

        // Hide assets
        sut = sut.hideAsset(AssetAddress.Fungible(ResourceAddress.sampleMainnet()))
            .hideAsset(AssetAddress.Fungible(ResourceAddress.sampleStokenet()))
            .hideAsset(AssetAddress.NonFungible(NonFungibleGlobalId.sample()))
            .hideAsset(AssetAddress.PoolUnit(PoolAddress.sampleMainnet()))

        assertEquals(
            listOf(
                AssetAddress.Fungible(ResourceAddress.sampleMainnet()),
                AssetAddress.Fungible(ResourceAddress.sampleStokenet()),
                AssetAddress.NonFungible(NonFungibleGlobalId.sample()),
                AssetAddress.PoolUnit(PoolAddress.sampleMainnet())
            ),
            sut.hiddenAssets
        )

        // Unhide assets
        sut = sut.unhideAsset(AssetAddress.Fungible(ResourceAddress.sampleStokenet()))
            .unhideAsset(AssetAddress.NonFungible(NonFungibleGlobalId.sample()))

        assertEquals(
            listOf(
                AssetAddress.Fungible(ResourceAddress.sampleMainnet()),
                AssetAddress.PoolUnit(PoolAddress.sampleMainnet())
            ),
            sut.hiddenAssets
        )
    }
}