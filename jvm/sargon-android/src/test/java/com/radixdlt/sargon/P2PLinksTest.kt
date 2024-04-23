package com.radixdlt.sargon

import com.radixdlt.sargon.extensions.append
import com.radixdlt.sargon.extensions.contains
import com.radixdlt.sargon.extensions.get
import com.radixdlt.sargon.extensions.getBy
import com.radixdlt.sargon.extensions.id
import com.radixdlt.sargon.extensions.init
import com.radixdlt.sargon.extensions.invoke
import com.radixdlt.sargon.extensions.remove
import com.radixdlt.sargon.extensions.removeById
import com.radixdlt.sargon.extensions.size
import com.radixdlt.sargon.extensions.updateOrAppend
import com.radixdlt.sargon.extensions.updateOrInsert
import com.radixdlt.sargon.samples.sample
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Assertions.assertFalse
import org.junit.jupiter.api.Assertions.assertTrue
import org.junit.jupiter.api.Test

class P2PLinksTest {

    @Test
    fun testListMethods() {
        val sample = P2pLink.sample()
        val sampleOther = P2pLink.sample.other()

        var list = P2pLinks.init(sample)

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
        assertEquals(sampleOther, list()[0])
        assertTrue(list.size == 2)
        list = list.updateOrAppend(sampleOther)
        assertTrue(list.size == 2)
        list = list.remove(sampleOther)

        assertEquals(sample, list.getBy(sample.id))
        assertTrue(list.removeById(sample.id).size == 0)
    }
}