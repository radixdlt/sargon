package com.radixdlt.sargon

import com.radixdlt.sargon.extensions.clientID
import com.radixdlt.sargon.extensions.fromJson
import com.radixdlt.sargon.extensions.hex
import com.radixdlt.sargon.extensions.init
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

    @Test
    fun testClientID() {
        val sut = P2pLink.sample.invoke()
        Assertions.assertEquals(
            Hash.init("98e140d9c01c069aa927797627b1bca4d25971a76549ca59df8ef9d8397afa97"),
            sut.clientID()
        )
    }
}