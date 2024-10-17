package com.radixdlt.sargon.samples

import com.radixdlt.sargon.annotation.UsesSampleValues
import com.radixdlt.sargon.SignedTransactionIntentHash
import com.radixdlt.sargon.newSignedIntentHashSample
import com.radixdlt.sargon.newSignedIntentHashSampleOther

@UsesSampleValues
val SignedTransactionIntentHash.Companion.sample: Sample<SignedTransactionIntentHash>
    get() = object : Sample<SignedTransactionIntentHash> {

        override fun invoke(): SignedTransactionIntentHash = newSignedIntentHashSample()

        override fun other(): SignedTransactionIntentHash = newSignedIntentHashSampleOther()
    }