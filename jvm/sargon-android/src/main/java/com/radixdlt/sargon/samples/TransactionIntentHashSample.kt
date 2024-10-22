package com.radixdlt.sargon.samples

import com.radixdlt.sargon.annotation.UsesSampleValues
import com.radixdlt.sargon.TransactionIntentHash
import com.radixdlt.sargon.newIntentHashSample
import com.radixdlt.sargon.newIntentHashSampleOther

@UsesSampleValues
val TransactionIntentHash.Companion.sample: Sample<TransactionIntentHash>
    get() = object : Sample<TransactionIntentHash> {

        override fun invoke(): TransactionIntentHash = newIntentHashSample()

        override fun other(): TransactionIntentHash = newIntentHashSampleOther()
    }