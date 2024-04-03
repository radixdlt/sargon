package com.radixdlt.sargon

import com.radixdlt.sargon.extensions.address
import com.radixdlt.sargon.samples.Sample
import com.radixdlt.sargon.samples.sample
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Assertions.assertInstanceOf
import org.junit.jupiter.api.Test

class ResourceIndicatorTest: SampleTestable<ResourceIndicator> {
    override val samples: List<Sample<ResourceIndicator>>
        get() = listOf(ResourceIndicator.sample)

    @Test
    fun testIds() {
        val sample = ResourceIndicator.sample()
        assertEquals(
            (sample as ResourceIndicator.Fungible).resourceAddress,
            sample.address
        )
        assertInstanceOf(ResourceIndicator.Fungible::class.java, sample)
    }
}