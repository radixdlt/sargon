package com.radixdlt.sargon.samples

import androidx.annotation.VisibleForTesting
import com.radixdlt.sargon.TransactionIntent
import com.radixdlt.sargon.newTransactionIntentSample
import com.radixdlt.sargon.newTransactionIntentSampleOther

@VisibleForTesting
val TransactionIntent.Companion.sample: Sample<TransactionIntent>
    get() = object : Sample<TransactionIntent> {

        override fun invoke(): TransactionIntent = newTransactionIntentSample()

        override fun other(): TransactionIntent = newTransactionIntentSampleOther()
    }