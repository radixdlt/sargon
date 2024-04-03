package com.radixdlt.sargon.samples

import com.radixdlt.sargon.annotation.UsesSampleValues
import com.radixdlt.sargon.Nonce
import com.radixdlt.sargon.newNonceSample
import com.radixdlt.sargon.newNonceSampleOther

@UsesSampleValues
val Nonce.Companion.sample: Sample<Nonce>
    get() = object : Sample<Nonce> {

        override fun invoke(): Nonce = newNonceSample()

        override fun other(): Nonce = newNonceSampleOther()
    }