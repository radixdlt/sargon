package com.radixdlt.sargon.samples

import com.radixdlt.sargon.annotation.UsesSampleValues
import com.radixdlt.sargon.TransactionIntentHash
import com.radixdlt.sargon.newTransactionIntentHashSample
import com.radixdlt.sargon.newTransactionIntentHashSampleOther

@UsesSampleValues
val TransactionIntentHash.Companion.sample: Sample<TransactionIntentHash>
    get() = object : Sample<TransactionIntentHash> {

        override fun invoke(): TransactionIntentHash = newTransactionIntentHashSample()

        override fun other(): TransactionIntentHash = newTransactionIntentHashSampleOther()
    }