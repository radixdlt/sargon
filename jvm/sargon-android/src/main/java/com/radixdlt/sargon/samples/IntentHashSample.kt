package com.radixdlt.sargon.samples

import androidx.annotation.VisibleForTesting
import com.radixdlt.sargon.IntentHash
import com.radixdlt.sargon.newIntentHashSample
import com.radixdlt.sargon.newIntentHashSampleOther

@VisibleForTesting
val IntentHash.Companion.sample: Sample<IntentHash>
    get() = object : Sample<IntentHash> {

        override fun invoke(): IntentHash = newIntentHashSample()

        override fun other(): IntentHash = newIntentHashSampleOther()
    }