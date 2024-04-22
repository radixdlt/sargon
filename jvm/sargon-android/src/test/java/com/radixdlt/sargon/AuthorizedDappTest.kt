package com.radixdlt.sargon

import com.radixdlt.sargon.extensions.deserializeFromJsonString
import com.radixdlt.sargon.extensions.serializedJsonString
import com.radixdlt.sargon.samples.Sample
import com.radixdlt.sargon.samples.sampleMainnet
import com.radixdlt.sargon.samples.sampleStokenet
import org.junit.jupiter.api.Assertions
import org.junit.jupiter.api.Test

class AuthorizedDappTest: SampleTestable<AuthorizedDapp> {
    override val samples: List<Sample<AuthorizedDapp>>
        get() = listOf(AuthorizedDapp.sampleMainnet, AuthorizedDapp.sampleStokenet)

    @Test
    fun testJsonRoundtrip() {
        val sut = AuthorizedDapp.sampleMainnet()
        Assertions.assertEquals(
            sut,
            AuthorizedDapp.deserializeFromJsonString(sut.serializedJsonString())
        )
    }
}