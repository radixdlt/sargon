package com.radixdlt.sargon.samples

import androidx.annotation.VisibleForTesting
import com.radixdlt.sargon.Nonce
import com.radixdlt.sargon.newNonceSample
import com.radixdlt.sargon.newNonceSampleOther

@VisibleForTesting
val Nonce.Companion.sample: Sample<Nonce>
    get() = object : Sample<Nonce> {

        override fun invoke(): Nonce = newNonceSample()

        override fun other(): Nonce = newNonceSampleOther()
    }