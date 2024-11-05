package com.radixdlt.sargon

import com.radixdlt.sargon.extensions.formatted
import com.radixdlt.sargon.extensions.init
import com.radixdlt.sargon.samples.Sample
import com.radixdlt.sargon.samples.sample
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Test

class SubintentHashTest : SampleTestable<SubintentHash> {

    override val samples: List<Sample<SubintentHash>>
        get() = listOf(SubintentHash.sample)

    @Test
    fun test() {
        val sut = SubintentHash.init("subtxid_rdx1frcm6zzyfd08z0deu9x24sh64eccxeux4j2dv3dsqeuh9qsz4y6sy6hgte")
        val actual = sut.formatted()

        assertEquals("subt...y6hgte", actual)
    }
}