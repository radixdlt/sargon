package com.radixdlt.sargon

import com.radixdlt.sargon.samples.Sample
import com.radixdlt.sargon.samples.sampleMainnet
import com.radixdlt.sargon.samples.sampleStokenet

class AuthorizedDappTest: SampleTestable<AuthorizedDapp> {
    override val samples: List<Sample<AuthorizedDapp>>
        get() = listOf(AuthorizedDapp.sampleMainnet, AuthorizedDapp.sampleStokenet)
}