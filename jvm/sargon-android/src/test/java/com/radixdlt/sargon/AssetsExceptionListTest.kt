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
import org.junit.jupiter.api.Assertions
import org.junit.jupiter.api.Test

class AssetsExceptionListTest : SampleTestable<AssetsExceptionList> {
    override val samples: List<Sample<AssetsExceptionList>>
        get() = listOf(AssetsExceptionList.sample)

    @Test
    fun testListMethods() {
        val sample = AssetException.sample()
        val sampleOther = AssetException.sample.other()

        var list = AssetsExceptionList.init(sample)

        Assertions.assertTrue(sample in list)
        Assertions.assertEquals(1, list.size)
        Assertions.assertEquals(sample, list[0])

        list = list.append(sampleOther)
        Assertions.assertTrue(sampleOther in list)
        Assertions.assertEquals(2, list.size)
        Assertions.assertEquals(sampleOther, list[1])

        list = list.remove(sampleOther)
        Assertions.assertFalse(sampleOther in list)
        Assertions.assertEquals(1, list.size)

        list = list.updateOrInsert(sampleOther, 0)
        Assertions.assertEquals(sampleOther, list()[0])
        Assertions.assertTrue(list.size == 2)
        list = list.updateOrAppend(sampleOther)
        Assertions.assertTrue(list.size == 2)
        list = list.remove(sampleOther)

        Assertions.assertEquals(sample, list.getBy(sample.address))
        Assertions.assertTrue(list.removeByAddress(sample.address).size == 0)
    }
}