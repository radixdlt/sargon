package com.radixdlt.sargon.samples

import androidx.annotation.VisibleForTesting
import androidx.compose.ui.tooling.preview.PreviewParameterProvider
import com.radixdlt.sargon.CompiledNotarizedIntent
import com.radixdlt.sargon.NotarySignature
import com.radixdlt.sargon.SargonBuildInformation
import com.radixdlt.sargon.newCompiledNotarizedIntentSample
import com.radixdlt.sargon.newCompiledNotarizedIntentSampleOther
import com.radixdlt.sargon.newNotarySignatureSample
import com.radixdlt.sargon.newNotarySignatureSampleOther
import com.radixdlt.sargon.newSargonBuildInformationSample
import com.radixdlt.sargon.newSargonBuildInformationSampleOther

@VisibleForTesting
val NotarySignature.Companion.sample: Sample<NotarySignature>
    get() = object : Sample<NotarySignature> {

        override fun invoke(): NotarySignature = newNotarySignatureSample()

        override fun other(): NotarySignature = newNotarySignatureSampleOther()
    }