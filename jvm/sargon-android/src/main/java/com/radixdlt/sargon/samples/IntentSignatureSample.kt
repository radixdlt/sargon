package com.radixdlt.sargon.samples

import androidx.annotation.VisibleForTesting
import androidx.compose.ui.tooling.preview.PreviewParameterProvider
import com.radixdlt.sargon.IntentSignature
import com.radixdlt.sargon.SargonBuildInformation
import com.radixdlt.sargon.newIntentSignatureSample
import com.radixdlt.sargon.newIntentSignatureSampleOther
import com.radixdlt.sargon.newSargonBuildInformationSample
import com.radixdlt.sargon.newSargonBuildInformationSampleOther

@VisibleForTesting
val IntentSignature.Companion.sample: Sample<IntentSignature>
    get() = object : Sample<IntentSignature> {

        override fun invoke(): IntentSignature = newIntentSignatureSample()

        override fun other(): IntentSignature = newIntentSignatureSampleOther()
    }