package com.radixdlt.sargon.samples

import androidx.annotation.VisibleForTesting
import androidx.compose.ui.tooling.preview.PreviewParameterProvider
import com.radixdlt.sargon.Hash
import com.radixdlt.sargon.IntentHash
import com.radixdlt.sargon.IntentSignature
import com.radixdlt.sargon.SargonBuildInformation
import com.radixdlt.sargon.SignedIntent
import com.radixdlt.sargon.newHashSample
import com.radixdlt.sargon.newHashSampleOther
import com.radixdlt.sargon.newIntentHashSample
import com.radixdlt.sargon.newIntentHashSampleOther
import com.radixdlt.sargon.newIntentSignatureSample
import com.radixdlt.sargon.newIntentSignatureSampleOther
import com.radixdlt.sargon.newSargonBuildInformationSample
import com.radixdlt.sargon.newSargonBuildInformationSampleOther
import com.radixdlt.sargon.newSignedIntentSample
import com.radixdlt.sargon.newSignedIntentSampleOther

@VisibleForTesting
val SignedIntent.Companion.sample: Sample<SignedIntent>
    get() = object : Sample<SignedIntent> {

        override fun invoke(): SignedIntent = newSignedIntentSample()

        override fun other(): SignedIntent = newSignedIntentSampleOther()
    }

class SignedIntentPreviewParameterProvider: PreviewParameterProvider<SignedIntent> {
    override val values: Sequence<SignedIntent>
        get() = SignedIntent.sample.all.asSequence()

}