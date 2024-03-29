package com.radixdlt.sargon.samples

import androidx.annotation.VisibleForTesting
import androidx.compose.ui.tooling.preview.PreviewParameterProvider
import com.radixdlt.sargon.FactorSource
import com.radixdlt.sargon.newFactorSourcesSample
import com.radixdlt.sargon.newFactorSourcesSampleOther

@VisibleForTesting
fun factorSourcesSample() = newFactorSourcesSample()

@VisibleForTesting
fun factorSourcesSampleOther() = newFactorSourcesSampleOther()

class FactorSourcesPreviewParameterProvider : PreviewParameterProvider<List<FactorSource>> {
    override val values: Sequence<List<FactorSource>>
        get() = sequenceOf(factorSourcesSample(), factorSourcesSampleOther())
}