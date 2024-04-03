package com.radixdlt.sargon.samples

import com.radixdlt.sargon.annotation.UsesSampleValues
import com.radixdlt.sargon.NotarySignature
import com.radixdlt.sargon.newNotarySignatureSample
import com.radixdlt.sargon.newNotarySignatureSampleOther

@UsesSampleValues
val NotarySignature.Companion.sample: Sample<NotarySignature>
    get() = object : Sample<NotarySignature> {

        override fun invoke(): NotarySignature = newNotarySignatureSample()

        override fun other(): NotarySignature = newNotarySignatureSampleOther()
    }