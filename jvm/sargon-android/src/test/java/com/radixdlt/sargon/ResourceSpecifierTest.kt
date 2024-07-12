package com.radixdlt.sargon

import com.radixdlt.sargon.extensions.address
import com.radixdlt.sargon.samples.Sample
import com.radixdlt.sargon.samples.sample
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Assertions.assertInstanceOf
import org.junit.jupiter.api.Test

class ResourceSpecifierTest: SampleTestable<ResourceSpecifier> {
    override val samples: List<Sample<ResourceSpecifier>>
        get() = listOf(ResourceSpecifier.sample)

    @Test
    fun testIds() {
        val sample = ResourceSpecifier.sample()
        assertEquals(
            (sample as ResourceSpecifier.Fungible).resourceAddress,
            sample.address
        )
        assertInstanceOf(ResourceSpecifier.Fungible::class.java, sample)
    }
}
