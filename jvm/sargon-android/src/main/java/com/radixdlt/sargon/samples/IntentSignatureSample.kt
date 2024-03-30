package com.radixdlt.sargon.samples

import androidx.annotation.VisibleForTesting
import com.radixdlt.sargon.IntentSignature
import com.radixdlt.sargon.newIntentSignatureSample
import com.radixdlt.sargon.newIntentSignatureSampleOther

@VisibleForTesting
val IntentSignature.Companion.sample: Sample<IntentSignature>
    get() = object : Sample<IntentSignature> {

        override fun invoke(): IntentSignature = newIntentSignatureSample()

        override fun other(): IntentSignature = newIntentSignatureSampleOther()
    }