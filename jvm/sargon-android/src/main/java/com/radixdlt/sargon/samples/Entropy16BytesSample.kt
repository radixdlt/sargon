package com.radixdlt.sargon.samples

import com.radixdlt.sargon.Entropy16Bytes
import com.radixdlt.sargon.annotation.UsesSampleValues
import com.radixdlt.sargon.newEntropy16BytesSample
import com.radixdlt.sargon.newEntropy16BytesSampleOther

@UsesSampleValues
val Entropy16Bytes.Companion.sample: Sample<Entropy16Bytes>
    get() = object : Sample<Entropy16Bytes> {
        override fun invoke(): Entropy16Bytes = newEntropy16BytesSample()

        override fun other(): Entropy16Bytes = newEntropy16BytesSampleOther()
    }