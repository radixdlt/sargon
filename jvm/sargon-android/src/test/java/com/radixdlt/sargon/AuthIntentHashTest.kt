package com.radixdlt.sargon

import com.radixdlt.sargon.samples.Sample
import com.radixdlt.sargon.samples.sample

class AuthIntentHashTest: SampleTestable<AuthIntentHash> {
    override val samples: List<Sample<AuthIntentHash>>
        get() = listOf(AuthIntentHash.sample)
}