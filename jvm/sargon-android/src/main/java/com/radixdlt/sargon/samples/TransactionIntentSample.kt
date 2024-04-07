package com.radixdlt.sargon.samples

import com.radixdlt.sargon.annotation.UsesSampleValues
import com.radixdlt.sargon.TransactionIntent
import com.radixdlt.sargon.newTransactionIntentSample
import com.radixdlt.sargon.newTransactionIntentSampleOther

@UsesSampleValues
val TransactionIntent.Companion.sample: Sample<TransactionIntent>
    get() = object : Sample<TransactionIntent> {

        override fun invoke(): TransactionIntent = newTransactionIntentSample()

        override fun other(): TransactionIntent = newTransactionIntentSampleOther()
    }