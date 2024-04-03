package com.radixdlt.sargon.samples

import com.radixdlt.sargon.annotation.UsesSampleValues
import com.radixdlt.sargon.IntentHash
import com.radixdlt.sargon.newIntentHashSample
import com.radixdlt.sargon.newIntentHashSampleOther

@UsesSampleValues
val IntentHash.Companion.sample: Sample<IntentHash>
    get() = object : Sample<IntentHash> {

        override fun invoke(): IntentHash = newIntentHashSample()

        override fun other(): IntentHash = newIntentHashSampleOther()
    }