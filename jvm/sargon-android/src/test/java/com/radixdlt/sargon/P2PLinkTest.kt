package com.radixdlt.sargon

import com.radixdlt.sargon.extensions.fromJson
import com.radixdlt.sargon.extensions.toJson
import com.radixdlt.sargon.samples.Sample
import com.radixdlt.sargon.samples.sample
import org.junit.jupiter.api.Assertions
import org.junit.jupiter.api.Test

class P2PLinkTest : SampleTestable<P2pLink> {

    override val samples: List<Sample<P2pLink>>
        get() = listOf(P2pLink.sample)

    @Test
    fun testJsonRoundtrip() {
        val sut = P2pLink.sample.invoke()
        Assertions.assertEquals(
            sut,
            P2pLink.fromJson(json = sut.toJson())
        )
    }
}