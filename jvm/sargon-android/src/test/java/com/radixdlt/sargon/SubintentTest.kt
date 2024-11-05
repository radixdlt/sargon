package com.radixdlt.sargon

import com.radixdlt.sargon.extensions.hash
import com.radixdlt.sargon.extensions.init
import com.radixdlt.sargon.samples.Sample
import com.radixdlt.sargon.samples.sample
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Test

class SubintentTest : SampleTestable<Subintent> {

    override val samples: List<Sample<Subintent>>
        get() = listOf(Subintent.sample)

    @Test
    fun test() {
        assertEquals(
            SubintentHash.init("subtxid_rdx1xput628m2l7jjweefd70gnq3t3a5x2gjeljduwm7vwly94s8ullql92sa0"),
            Subintent.sample().hash()
        )
    }
}