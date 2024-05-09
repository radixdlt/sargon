package com.radixdlt.sargon.samples

import com.radixdlt.sargon.Exactly32Bytes
import com.radixdlt.sargon.annotation.UsesSampleValues
import com.radixdlt.sargon.newExactly32BytesSample
import com.radixdlt.sargon.newExactly32BytesSampleOther

@UsesSampleValues
val Exactly32Bytes.Companion.sample: Sample<Exactly32Bytes>
    get() = object : Sample<Exactly32Bytes> {
        override fun invoke(): Exactly32Bytes = newExactly32BytesSample()

        override fun other(): Exactly32Bytes = newExactly32BytesSampleOther()
    }