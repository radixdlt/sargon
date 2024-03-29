package com.radixdlt.sargon.samples

import androidx.annotation.VisibleForTesting
import com.radixdlt.sargon.NotarySignature
import com.radixdlt.sargon.newNotarySignatureSample
import com.radixdlt.sargon.newNotarySignatureSampleOther

@VisibleForTesting
val NotarySignature.Companion.sample: Sample<NotarySignature>
    get() = object : Sample<NotarySignature> {

        override fun invoke(): NotarySignature = newNotarySignatureSample()

        override fun other(): NotarySignature = newNotarySignatureSampleOther()
    }