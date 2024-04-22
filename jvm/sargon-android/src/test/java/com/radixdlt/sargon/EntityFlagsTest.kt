package com.radixdlt.sargon

import com.radixdlt.sargon.extensions.append
import com.radixdlt.sargon.extensions.contains
import com.radixdlt.sargon.extensions.get
import com.radixdlt.sargon.extensions.init
import com.radixdlt.sargon.extensions.invoke
import com.radixdlt.sargon.extensions.remove
import com.radixdlt.sargon.extensions.size
import com.radixdlt.sargon.extensions.updateOrAppend
import com.radixdlt.sargon.extensions.updateOrInsert
import com.radixdlt.sargon.samples.Sample
import com.radixdlt.sargon.samples.sample
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Assertions.assertFalse
import org.junit.jupiter.api.Assertions.assertTrue
import org.junit.jupiter.api.Test

class EntityFlagsTest: SampleTestable<EntityFlags> {
    override val samples: List<Sample<EntityFlags>>
        get() = listOf(EntityFlags.sample)

    @Test
    fun testListMethods() {
        val sample = EntityFlag.DELETED_BY_USER
        val sampleOther = EntityFlag.PLACEHOLDER_SAMPLE_VALUE_FLAG

        var list = EntityFlags.init(sample)

        assertTrue(sample in list)
        assertEquals(1, list.size)
        assertEquals(sample, list[0])

        list = list.append(sampleOther)
        assertTrue(sampleOther in list)
        assertEquals(2, list.size)
        assertEquals(sampleOther, list.get(sampleOther))

        list = list.updateOrInsert(sampleOther, 0)
        assertEquals(sampleOther, list()[1])
        assertTrue(list.size == 2)
        list = list.updateOrAppend(sampleOther)
        assertTrue(list.size == 2)
        list = list.remove(sampleOther)

        list = list.remove(sampleOther)
        assertFalse(sampleOther in list)
        assertEquals(1, list.size)
    }
}