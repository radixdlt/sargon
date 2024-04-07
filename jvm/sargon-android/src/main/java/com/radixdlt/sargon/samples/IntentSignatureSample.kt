package com.radixdlt.sargon.samples

import com.radixdlt.sargon.annotation.UsesSampleValues
import com.radixdlt.sargon.IntentSignature
import com.radixdlt.sargon.newIntentSignatureSample
import com.radixdlt.sargon.newIntentSignatureSampleOther

@UsesSampleValues
val IntentSignature.Companion.sample: Sample<IntentSignature>
    get() = object : Sample<IntentSignature> {

        override fun invoke(): IntentSignature = newIntentSignatureSample()

        override fun other(): IntentSignature = newIntentSignatureSampleOther()
    }