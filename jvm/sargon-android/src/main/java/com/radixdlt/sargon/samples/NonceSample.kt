package com.radixdlt.sargon.samples

import androidx.annotation.VisibleForTesting
import androidx.compose.ui.tooling.preview.PreviewParameterProvider
import com.radixdlt.sargon.IntentSignature
import com.radixdlt.sargon.Nonce
import com.radixdlt.sargon.SargonBuildInformation
import com.radixdlt.sargon.newIntentSignatureSample
import com.radixdlt.sargon.newIntentSignatureSampleOther
import com.radixdlt.sargon.newNonceRandom
import com.radixdlt.sargon.newNonceSample
import com.radixdlt.sargon.newNonceSampleOther
import com.radixdlt.sargon.newSargonBuildInformationSample
import com.radixdlt.sargon.newSargonBuildInformationSampleOther

@VisibleForTesting
val Nonce.Companion.sample: Sample<Nonce>
    get() = object : Sample<Nonce> {

        override fun invoke(): Nonce = newNonceSample()

        override fun other(): Nonce = newNonceSampleOther()
    }