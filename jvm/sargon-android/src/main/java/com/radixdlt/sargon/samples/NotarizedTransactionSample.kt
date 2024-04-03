package com.radixdlt.sargon.samples

import com.radixdlt.sargon.annotation.UsesSampleValues
import com.radixdlt.sargon.NotarizedTransaction
import com.radixdlt.sargon.newNotarizedTransactionSample
import com.radixdlt.sargon.newNotarizedTransactionSampleOther

@UsesSampleValues
val NotarizedTransaction.Companion.sample: Sample<NotarizedTransaction>
    get() = object : Sample<NotarizedTransaction> {

        override fun invoke(): NotarizedTransaction = newNotarizedTransactionSample()

        override fun other(): NotarizedTransaction = newNotarizedTransactionSampleOther()
    }