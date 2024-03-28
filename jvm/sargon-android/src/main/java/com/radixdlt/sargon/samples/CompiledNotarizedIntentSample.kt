package com.radixdlt.sargon.samples

import androidx.annotation.VisibleForTesting
import androidx.compose.ui.tooling.preview.PreviewParameterProvider
import com.radixdlt.sargon.CompiledNotarizedIntent
import com.radixdlt.sargon.SargonBuildInformation
import com.radixdlt.sargon.newCompiledNotarizedIntentSample
import com.radixdlt.sargon.newCompiledNotarizedIntentSampleOther
import com.radixdlt.sargon.newSargonBuildInformationSample
import com.radixdlt.sargon.newSargonBuildInformationSampleOther

@VisibleForTesting
val CompiledNotarizedIntent.Companion.sample: Sample<CompiledNotarizedIntent>
    get() = object : Sample<CompiledNotarizedIntent> {

        override fun invoke(): CompiledNotarizedIntent = newCompiledNotarizedIntentSample()

        override fun other(): CompiledNotarizedIntent = newCompiledNotarizedIntentSampleOther()
    }

class CompiledNotarizedIntentPreviewParameterProvider: PreviewParameterProvider<CompiledNotarizedIntent> {
    override val values: Sequence<CompiledNotarizedIntent>
        get() = CompiledNotarizedIntent.sample.all.asSequence()

}