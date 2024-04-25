package com.radixdlt.sargon

import com.radixdlt.sargon.extensions.append
import com.radixdlt.sargon.extensions.contains
import com.radixdlt.sargon.extensions.get
import com.radixdlt.sargon.extensions.getBy
import com.radixdlt.sargon.extensions.init
import com.radixdlt.sargon.extensions.invoke
import com.radixdlt.sargon.extensions.remove
import com.radixdlt.sargon.extensions.removeByAddress
import com.radixdlt.sargon.extensions.size
import com.radixdlt.sargon.extensions.updateOrAppend
import com.radixdlt.sargon.extensions.updateOrInsert
import com.radixdlt.sargon.samples.Sample
import com.radixdlt.sargon.samples.sample
import com.radixdlt.sargon.samples.sampleMainnet
import com.radixdlt.sargon.samples.sampleStokenet
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Assertions.assertFalse
import org.junit.jupiter.api.Assertions.assertTrue
import org.junit.jupiter.api.Test

class DepositorsAllowListTest: SampleTestable<DepositorsAllowList> {
    override val samples: List<Sample<DepositorsAllowList>>
        get() = listOf(DepositorsAllowList.sample)

    @Test
    fun testListMethods() {
        val sample = ResourceOrNonFungible.sample()
        val sampleOther = ResourceOrNonFungible.sample.other()

        var list = DepositorsAllowList.init(sample)

        assertTrue(sample in list)
        assertEquals(1, list.size)
        assertEquals(sample, list[0])

        list = list.append(sampleOther)
        assertTrue(sampleOther in list)
        assertEquals(2, list.size)
        assertEquals(sampleOther, list[1])

        list = list.remove(sampleOther)
        assertFalse(sampleOther in list)
        assertEquals(1, list.size)

        list = list.updateOrInsert(sampleOther, 0)
        assertEquals(sampleOther, list.getBy(sampleOther))
        assertTrue(list.size == 2)
        list = list.updateOrAppend(sampleOther)
        assertTrue(list.size == 2)
    }
}