package com.radixdlt.sargon.samples

import androidx.annotation.VisibleForTesting
import androidx.compose.ui.tooling.preview.PreviewParameterProvider
import com.radixdlt.sargon.Hash
import com.radixdlt.sargon.IntentHash
import com.radixdlt.sargon.IntentSignature
import com.radixdlt.sargon.SargonBuildInformation
import com.radixdlt.sargon.SignedIntentHash
import com.radixdlt.sargon.newHashSample
import com.radixdlt.sargon.newHashSampleOther
import com.radixdlt.sargon.newIntentHashSample
import com.radixdlt.sargon.newIntentHashSampleOther
import com.radixdlt.sargon.newIntentSignatureSample
import com.radixdlt.sargon.newIntentSignatureSampleOther
import com.radixdlt.sargon.newSargonBuildInformationSample
import com.radixdlt.sargon.newSargonBuildInformationSampleOther
import com.radixdlt.sargon.newSignedIntentHashSample
import com.radixdlt.sargon.newSignedIntentHashSampleOther

@VisibleForTesting
val SignedIntentHash.Companion.sample: Sample<SignedIntentHash>
    get() = object : Sample<SignedIntentHash> {

        override fun invoke(): SignedIntentHash = newSignedIntentHashSample()

        override fun other(): SignedIntentHash = newSignedIntentHashSampleOther()
    }