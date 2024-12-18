package com.radixdlt.sargon.samples

import com.radixdlt.sargon.AuthIntentHash
import com.radixdlt.sargon.annotation.UsesSampleValues
import com.radixdlt.sargon.newAuthIntentHashSample
import com.radixdlt.sargon.newAuthIntentHashSampleOther

@UsesSampleValues
val AuthIntentHash.Companion.sample: Sample<AuthIntentHash>
    get() = object : Sample<AuthIntentHash> {
        override fun invoke(): AuthIntentHash = newAuthIntentHashSample()

        override fun other(): AuthIntentHash = newAuthIntentHashSampleOther()
    }