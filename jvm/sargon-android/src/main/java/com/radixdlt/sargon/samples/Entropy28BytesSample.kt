package com.radixdlt.sargon.samples

import com.radixdlt.sargon.Entropy28Bytes
import com.radixdlt.sargon.annotation.UsesSampleValues
import com.radixdlt.sargon.newEntropy28BytesSample
import com.radixdlt.sargon.newEntropy28BytesSampleOther

@UsesSampleValues
val Entropy28Bytes.Companion.sample: Sample<Entropy28Bytes>
    get() = object : Sample<Entropy28Bytes> {
        override fun invoke(): Entropy28Bytes = newEntropy28BytesSample()

        override fun other(): Entropy28Bytes = newEntropy28BytesSampleOther()
    }