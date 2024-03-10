package com.radixdlt.sargon.sample

import androidx.annotation.VisibleForTesting
import androidx.compose.ui.tooling.preview.PreviewParameterProvider
import com.radixdlt.sargon.TransactionManifest
import com.radixdlt.sargon.newTransactionManifestSample
import com.radixdlt.sargon.newTransactionManifestSampleOther

@VisibleForTesting
val TransactionManifest.Companion.sample: Sample<TransactionManifest>
    get() = object : Sample<TransactionManifest> {

        override fun invoke(): TransactionManifest = newTransactionManifestSample()

        override fun other(): TransactionManifest = newTransactionManifestSampleOther()

    }

class TransactionManifestPreviewParameterProvider: PreviewParameterProvider<TransactionManifest> {
    override val values: Sequence<TransactionManifest>
        get() = TransactionManifest.sample.all.asSequence()

}