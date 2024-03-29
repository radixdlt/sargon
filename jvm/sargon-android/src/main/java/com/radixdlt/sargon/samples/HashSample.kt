package com.radixdlt.sargon.samples

import androidx.annotation.VisibleForTesting
import com.radixdlt.sargon.Hash
import com.radixdlt.sargon.newHashSample
import com.radixdlt.sargon.newHashSampleOther

@VisibleForTesting
val Hash.Companion.sample: Sample<Hash>
    get() = object : Sample<Hash> {

        override fun invoke(): Hash = newHashSample()

        override fun other(): Hash = newHashSampleOther()
    }