package com.radixdlt.sargon.samples

import com.radixdlt.sargon.IntentDiscriminator
import com.radixdlt.sargon.annotation.UsesSampleValues
import com.radixdlt.sargon.newIntentDiscriminatorSample
import com.radixdlt.sargon.newIntentDiscriminatorSampleOther

@UsesSampleValues
val IntentDiscriminator.Companion.sample: Sample<IntentDiscriminator>
    get() = object : Sample<IntentDiscriminator> {
        override fun invoke(): IntentDiscriminator = newIntentDiscriminatorSample()

        override fun other(): IntentDiscriminator = newIntentDiscriminatorSampleOther()

    }