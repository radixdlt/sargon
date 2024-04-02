package com.radixdlt.sargon.samples

import com.radixdlt.sargon.annotation.UsesSampleValues
import com.radixdlt.sargon.PublicKeyHash
import com.radixdlt.sargon.newPublicKeyHashSample
import com.radixdlt.sargon.newPublicKeyHashSampleOther

@UsesSampleValues
val PublicKeyHash.Companion.sample: Sample<PublicKeyHash>
    get() = object : Sample<PublicKeyHash> {

        override fun invoke(): PublicKeyHash = newPublicKeyHashSample()

        override fun other(): PublicKeyHash = newPublicKeyHashSampleOther()
    }