package com.radixdlt.sargon.samples

import androidx.annotation.VisibleForTesting
import com.radixdlt.sargon.SignedIntent
import com.radixdlt.sargon.newSignedIntentSample
import com.radixdlt.sargon.newSignedIntentSampleOther

@VisibleForTesting
val SignedIntent.Companion.sample: Sample<SignedIntent>
    get() = object : Sample<SignedIntent> {

        override fun invoke(): SignedIntent = newSignedIntentSample()

        override fun other(): SignedIntent = newSignedIntentSampleOther()
    }