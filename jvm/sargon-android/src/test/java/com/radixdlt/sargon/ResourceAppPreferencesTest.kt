package com.radixdlt.sargon

import com.radixdlt.sargon.extensions.ResourceAppPreferences
import com.radixdlt.sargon.extensions.hiddenResources
import com.radixdlt.sargon.extensions.hideResource
import com.radixdlt.sargon.extensions.unhideResource
import com.radixdlt.sargon.samples.Sample
import com.radixdlt.sargon.samples.sample
import com.radixdlt.sargon.samples.sampleMainnet
import com.radixdlt.sargon.samples.sampleStokenet
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Assertions.assertTrue
import org.junit.jupiter.api.Test

class ResourceAppPreferencesTest : SampleTestable<ResourceAppPreferences> {

    override val samples: List<Sample<ResourceAppPreferences>>
        get() = listOf(ResourceAppPreferences.sample)

    @Test
    fun testHiddenAssets() {
        var sut = ResourceAppPreferences()

        assertTrue(sut.hiddenResources.isEmpty())

        // Hide assets
        sut = sut.hideResource(ResourceIdentifier.Fungible(ResourceAddress.sampleMainnet()))
            .hideResource(ResourceIdentifier.Fungible(ResourceAddress.sampleStokenet()))
            .hideResource(ResourceIdentifier.NonFungible(ResourceAddress.sampleMainnet()))
            .hideResource(ResourceIdentifier.PoolUnit(PoolAddress.sampleMainnet()))

        assertEquals(
            listOf(
                ResourceIdentifier.Fungible(ResourceAddress.sampleMainnet()),
                ResourceIdentifier.Fungible(ResourceAddress.sampleStokenet()),
                ResourceIdentifier.NonFungible(ResourceAddress.sampleMainnet()),
                ResourceIdentifier.PoolUnit(PoolAddress.sampleMainnet())
            ),
            sut.hiddenResources
        )

        // Unhide assets
        sut = sut.unhideResource(ResourceIdentifier.Fungible(ResourceAddress.sampleStokenet()))
            .unhideResource(ResourceIdentifier.NonFungible(ResourceAddress.sampleMainnet()))

        assertEquals(
            listOf(
                ResourceIdentifier.Fungible(ResourceAddress.sampleMainnet()),
                ResourceIdentifier.PoolUnit(PoolAddress.sampleMainnet())
            ),
            sut.hiddenResources
        )
    }
}