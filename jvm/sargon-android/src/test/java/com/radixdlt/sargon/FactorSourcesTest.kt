package com.radixdlt.sargon

import com.radixdlt.sargon.extensions.contains
import com.radixdlt.sargon.extensions.get
import com.radixdlt.sargon.extensions.init
import com.radixdlt.sargon.extensions.size
import com.radixdlt.sargon.samples.Sample
import com.radixdlt.sargon.samples.sample
import org.junit.jupiter.api.Assertions
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Test
import org.junit.jupiter.api.assertThrows

class FactorSourcesTest: SampleTestable<FactorSources> {

    override val samples: List<Sample<FactorSources>>
        get() = listOf(FactorSources.sample)

    @Test
    fun testListMethods() {
        val first = FactorSource.sample()
        val samples = FactorSources.init(first)

        Assertions.assertTrue(first in samples)
        assertEquals(
            1,
            samples.size
        )
        assertEquals(
            first,
            samples[0]
        )
    }

    @Test
    fun testEmptyFactorSourcesFails() {
        assertThrows<CommonException.FactorSourcesMustNotBeEmpty> {
            FactorSources.init()
        }
    }

}