package com.radixdlt.sargon.samples

import com.radixdlt.sargon.Entropy24Bytes
import com.radixdlt.sargon.annotation.UsesSampleValues
import com.radixdlt.sargon.newEntropy24BytesSample
import com.radixdlt.sargon.newEntropy24BytesSampleOther

@UsesSampleValues
val Entropy24Bytes.Companion.sample: Sample<Entropy24Bytes>
    get() = object : Sample<Entropy24Bytes> {
        override fun invoke(): Entropy24Bytes = newEntropy24BytesSample()

        override fun other(): Entropy24Bytes = newEntropy24BytesSampleOther()
    }