package com.radixdlt.sargon.samples

import androidx.annotation.VisibleForTesting
import com.radixdlt.sargon.PublicKeyHash
import com.radixdlt.sargon.newPublicKeyHashSample
import com.radixdlt.sargon.newPublicKeyHashSampleOther

@VisibleForTesting
val PublicKeyHash.Companion.sample: Sample<PublicKeyHash>
    get() = object : Sample<PublicKeyHash> {

        override fun invoke(): PublicKeyHash = newPublicKeyHashSample()

        override fun other(): PublicKeyHash = newPublicKeyHashSampleOther()
    }