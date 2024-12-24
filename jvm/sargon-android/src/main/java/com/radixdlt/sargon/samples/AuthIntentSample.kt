package com.radixdlt.sargon.samples

import com.radixdlt.sargon.AuthIntent
import com.radixdlt.sargon.annotation.UsesSampleValues
import com.radixdlt.sargon.newAuthIntentSample
import com.radixdlt.sargon.newAuthIntentSampleOther

@UsesSampleValues
val AuthIntent.Companion.sample: Sample<AuthIntent>
    get() = object : Sample<AuthIntent> {
        override fun invoke(): AuthIntent = newAuthIntentSample()

        override fun other(): AuthIntent = newAuthIntentSampleOther()
    }