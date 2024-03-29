package com.radixdlt.sargon.samples

import androidx.annotation.VisibleForTesting
import androidx.compose.ui.tooling.preview.PreviewParameterProvider
import com.radixdlt.sargon.Hash
import com.radixdlt.sargon.IntentSignature
import com.radixdlt.sargon.SargonBuildInformation
import com.radixdlt.sargon.newHashSample
import com.radixdlt.sargon.newHashSampleOther
import com.radixdlt.sargon.newIntentSignatureSample
import com.radixdlt.sargon.newIntentSignatureSampleOther
import com.radixdlt.sargon.newSargonBuildInformationSample
import com.radixdlt.sargon.newSargonBuildInformationSampleOther

@VisibleForTesting
val Hash.Companion.sample: Sample<Hash>
    get() = object : Sample<Hash> {

        override fun invoke(): Hash = newHashSample()

        override fun other(): Hash = newHashSampleOther()
    }