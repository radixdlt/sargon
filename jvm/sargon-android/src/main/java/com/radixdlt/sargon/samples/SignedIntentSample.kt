package com.radixdlt.sargon.samples

import com.radixdlt.sargon.annotation.UsesSampleValues
import com.radixdlt.sargon.SignedIntent
import com.radixdlt.sargon.newSignedIntentSample
import com.radixdlt.sargon.newSignedIntentSampleOther

@UsesSampleValues
val SignedIntent.Companion.sample: Sample<SignedIntent>
    get() = object : Sample<SignedIntent> {

        override fun invoke(): SignedIntent = newSignedIntentSample()

        override fun other(): SignedIntent = newSignedIntentSampleOther()
    }