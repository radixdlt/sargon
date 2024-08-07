package com.radixdlt.sargon.samples

import com.radixdlt.sargon.annotation.UsesSampleValues
import com.radixdlt.sargon.SignedIntentHash
import com.radixdlt.sargon.newSignedIntentHashSample
import com.radixdlt.sargon.newSignedIntentHashSampleOther

@UsesSampleValues
val SignedIntentHash.Companion.sample: Sample<SignedIntentHash>
    get() = object : Sample<SignedIntentHash> {

        override fun invoke(): SignedIntentHash = newSignedIntentHashSample()

        override fun other(): SignedIntentHash = newSignedIntentHashSampleOther()
    }