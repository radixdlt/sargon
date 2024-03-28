package com.radixdlt.sargon.samples

import androidx.annotation.VisibleForTesting
import androidx.compose.ui.tooling.preview.PreviewParameterProvider
import com.radixdlt.sargon.CompiledNotarizedIntent
import com.radixdlt.sargon.NotarizedTransaction
import com.radixdlt.sargon.SargonBuildInformation
import com.radixdlt.sargon.newCompiledNotarizedIntentSample
import com.radixdlt.sargon.newCompiledNotarizedIntentSampleOther
import com.radixdlt.sargon.newNotarizedTransactionSample
import com.radixdlt.sargon.newNotarizedTransactionSampleOther
import com.radixdlt.sargon.newSargonBuildInformationSample
import com.radixdlt.sargon.newSargonBuildInformationSampleOther

@VisibleForTesting
val NotarizedTransaction.Companion.sample: Sample<NotarizedTransaction>
    get() = object : Sample<NotarizedTransaction> {

        override fun invoke(): NotarizedTransaction = newNotarizedTransactionSample()

        override fun other(): NotarizedTransaction = newNotarizedTransactionSampleOther()
    }

class NotarizedTransactionPreviewParameterProvider: PreviewParameterProvider<NotarizedTransaction> {
    override val values: Sequence<NotarizedTransaction>
        get() = NotarizedTransaction.sample.all.asSequence()

}