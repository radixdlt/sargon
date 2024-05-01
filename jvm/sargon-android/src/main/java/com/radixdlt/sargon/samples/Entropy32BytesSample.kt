package com.radixdlt.sargon.samples

import com.radixdlt.sargon.Entropy32Bytes
import com.radixdlt.sargon.annotation.UsesSampleValues
import com.radixdlt.sargon.newEntropy32BytesSample
import com.radixdlt.sargon.newEntropy32BytesSampleOther

@UsesSampleValues
val Entropy32Bytes.Companion.sample: Sample<Entropy32Bytes>
    get() = object : Sample<Entropy32Bytes> {
        override fun invoke(): Entropy32Bytes = newEntropy32BytesSample()

        override fun other(): Entropy32Bytes = newEntropy32BytesSampleOther()
    }