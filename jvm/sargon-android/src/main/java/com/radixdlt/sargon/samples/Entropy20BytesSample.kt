package com.radixdlt.sargon.samples

import com.radixdlt.sargon.Entropy20Bytes
import com.radixdlt.sargon.annotation.UsesSampleValues
import com.radixdlt.sargon.newEntropy20BytesSample
import com.radixdlt.sargon.newEntropy20BytesSampleOther

@UsesSampleValues
val Entropy20Bytes.Companion.sample: Sample<Entropy20Bytes>
    get() = object : Sample<Entropy20Bytes> {
        override fun invoke(): Entropy20Bytes = newEntropy20BytesSample()

        override fun other(): Entropy20Bytes = newEntropy20BytesSampleOther()
    }