package com.radixdlt.sargon.samples

import com.radixdlt.sargon.annotation.UsesSampleValues
import com.radixdlt.sargon.Blobs
import com.radixdlt.sargon.newBlobsSample
import com.radixdlt.sargon.newBlobsSampleOther

@UsesSampleValues
val Blobs.Companion.sample: Sample<Blobs>
    get() = object : Sample<Blobs> {

        override fun invoke(): Blobs = newBlobsSample()

        override fun other(): Blobs = newBlobsSampleOther()
    }