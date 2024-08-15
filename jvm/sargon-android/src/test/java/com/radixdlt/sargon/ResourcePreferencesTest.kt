package com.radixdlt.sargon

import com.radixdlt.sargon.extensions.hiddenResources
import com.radixdlt.sargon.extensions.hide
import com.radixdlt.sargon.extensions.unhide
import com.radixdlt.sargon.samples.Sample
import com.radixdlt.sargon.samples.sample
import com.radixdlt.sargon.samples.sampleMainnet
import com.radixdlt.sargon.samples.sampleStokenet
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Assertions.assertTrue
import org.junit.jupiter.api.Test

class ResourcePreferencesTest : SampleTestable<ResourcePreferences> {

    override val samples: List<Sample<ResourcePreferences>>
        get() = listOf(ResourcePreferences.sample)

    @Test
    fun testHiddenResources() {
        var sut = ResourcePreferences(
            fungible = emptyMap(),
            nonFungible = emptyMap(),
            poolUnit = emptyMap()
        )

        assertTrue(sut.hiddenResources.fungible.isEmpty())
        assertTrue(sut.hiddenResources.nonFungible.isEmpty())
        assertTrue(sut.hiddenResources.poolUnit.isEmpty())

        // Hide resources
        sut = sut.hide(ResourcePreferenceKind.Fungible(ResourceAddress.sampleMainnet()))
            .hide(ResourcePreferenceKind.Fungible(ResourceAddress.sampleStokenet()))
            .hide(ResourcePreferenceKind.NonFungible(NonFungibleGlobalId.sample()))
            .hide(ResourcePreferenceKind.PoolUnit(PoolAddress.sampleMainnet()))

        assertEquals(listOf(ResourceAddress.sampleMainnet(), ResourceAddress.sampleStokenet()), sut.hiddenResources.fungible)
        assertEquals(listOf(NonFungibleGlobalId.sample()), sut.hiddenResources.nonFungible)
        assertEquals(listOf(PoolAddress.sampleMainnet()), sut.hiddenResources.poolUnit)

        // Unhide resources
        sut = sut.unhide(ResourcePreferenceKind.Fungible(ResourceAddress.sampleStokenet()))
            .unhide(ResourcePreferenceKind.NonFungible(NonFungibleGlobalId.sample()))

        assertEquals(listOf(ResourceAddress.sampleMainnet()), sut.hiddenResources.fungible)
        assertTrue(sut.hiddenResources.nonFungible.isEmpty())
        assertEquals(listOf(PoolAddress.sampleMainnet()), sut.hiddenResources.poolUnit)
    }
}