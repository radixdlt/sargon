package com.radixdlt.sargon.samples

import androidx.annotation.VisibleForTesting
import com.radixdlt.sargon.NotarizedTransaction
import com.radixdlt.sargon.newNotarizedTransactionSample
import com.radixdlt.sargon.newNotarizedTransactionSampleOther

@VisibleForTesting
val NotarizedTransaction.Companion.sample: Sample<NotarizedTransaction>
    get() = object : Sample<NotarizedTransaction> {

        override fun invoke(): NotarizedTransaction = newNotarizedTransactionSample()

        override fun other(): NotarizedTransaction = newNotarizedTransactionSampleOther()
    }