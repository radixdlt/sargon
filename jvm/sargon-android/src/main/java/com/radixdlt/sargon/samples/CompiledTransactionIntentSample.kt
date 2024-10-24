package com.radixdlt.sargon.samples

import com.radixdlt.sargon.CompiledTransactionIntent
import com.radixdlt.sargon.annotation.UsesSampleValues
import com.radixdlt.sargon.newCompiledTransactionIntentSample
import com.radixdlt.sargon.newCompiledTransactionIntentSampleOther

@UsesSampleValues
val CompiledTransactionIntent.Companion.sample
    get() = object : Sample<CompiledTransactionIntent> {
        override fun invoke(): CompiledTransactionIntent = newCompiledTransactionIntentSample()

        override fun other(): CompiledTransactionIntent = newCompiledTransactionIntentSampleOther()
    }