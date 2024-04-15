package com.radixdlt.sargon

import com.radixdlt.sargon.extensions.append
import com.radixdlt.sargon.extensions.contains
import com.radixdlt.sargon.extensions.get
import com.radixdlt.sargon.extensions.id
import com.radixdlt.sargon.extensions.init
import com.radixdlt.sargon.extensions.remove
import com.radixdlt.sargon.extensions.removeById
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
        val sample = FactorSource.sample()
        assertEquals(sample.id, FactorSource.sample().id)
        val sampleOther = FactorSource.sample.other()

        var list = FactorSources.init(sample)

        Assertions.assertTrue(sample in list)
        assertEquals(1, list.size)
        assertEquals(sample, list[0])

        list = list.append(sampleOther)
        Assertions.assertTrue(sampleOther in list)
        assertEquals(2, list.size)
        assertEquals(sampleOther, list[1])

        list = list.remove(sampleOther)
        Assertions.assertFalse(sampleOther in list)
        assertEquals(1, list.size)

        list = list.append(sampleOther)
        assertEquals(sampleOther, list.get(sampleOther.id))
        Assertions.assertTrue(list.removeById(sampleOther.id).size == 1)
    }

    @Test
    fun testEmptyFactorSourcesFails() {
        assertThrows<CommonException.FactorSourcesMustNotBeEmpty> {
            FactorSources.init()
        }
    }

}