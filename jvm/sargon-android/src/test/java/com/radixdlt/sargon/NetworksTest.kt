package com.radixdlt.sargon

import com.radixdlt.sargon.extensions.append
import com.radixdlt.sargon.extensions.contains
import com.radixdlt.sargon.extensions.get
import com.radixdlt.sargon.extensions.getBy
import com.radixdlt.sargon.extensions.init
import com.radixdlt.sargon.extensions.remove
import com.radixdlt.sargon.extensions.removeByNetworkId
import com.radixdlt.sargon.extensions.size
import com.radixdlt.sargon.samples.sampleMainnet
import com.radixdlt.sargon.samples.sampleStokenet
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Assertions.assertFalse
import org.junit.jupiter.api.Assertions.assertTrue
import org.junit.jupiter.api.Test

class NetworksTest  {

    @Test
    fun testListMethods() {
        val sample = ProfileNetwork.sampleMainnet()
        val sampleOther = ProfileNetwork.sampleStokenet()

        var list = ProfileNetworks.init(sample)

        assertTrue(sample in list)
        assertEquals(1, list.size)
        assertEquals(sample, list[0])

        list = list.append(sampleOther)
        assertTrue(sampleOther in list)
        assertEquals(2, list.size)
        assertEquals(sampleOther, list[1])

        list = list.remove(list.getBy(sampleOther.id)!!)
        assertFalse(sampleOther in list)
        assertEquals(1, list.size)

        list = list.append(sampleOther)
        list = list.removeByNetworkId(sampleOther.id)
        assertFalse(sampleOther in list)
        assertEquals(1, list.size)
    }
}