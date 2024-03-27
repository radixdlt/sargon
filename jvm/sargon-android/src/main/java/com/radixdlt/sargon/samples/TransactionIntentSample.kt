package com.radixdlt.sargon.samples

import androidx.annotation.VisibleForTesting
import androidx.compose.ui.tooling.preview.PreviewParameterProvider
import com.radixdlt.sargon.CompiledNotarizedIntent
import com.radixdlt.sargon.SargonBuildInformation
import com.radixdlt.sargon.TransactionIntent
import com.radixdlt.sargon.newCompiledNotarizedIntentSample
import com.radixdlt.sargon.newCompiledNotarizedIntentSampleOther
import com.radixdlt.sargon.newSargonBuildInformationSample
import com.radixdlt.sargon.newSargonBuildInformationSampleOther
import com.radixdlt.sargon.newTransactionIntentSample
import com.radixdlt.sargon.newTransactionIntentSampleOther

@VisibleForTesting
val TransactionIntent.Companion.sample: Sample<TransactionIntent>
    get() = object : Sample<TransactionIntent> {

        override fun invoke(): TransactionIntent = newTransactionIntentSample()

        override fun other(): TransactionIntent = newTransactionIntentSampleOther()
    }

class TransactionIntentPreviewParameterProvider: PreviewParameterProvider<TransactionIntent> {
    override val values: Sequence<TransactionIntent>
        get() = TransactionIntent.sample.all.asSequence()

}