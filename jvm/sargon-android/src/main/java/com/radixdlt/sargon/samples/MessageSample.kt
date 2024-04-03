package com.radixdlt.sargon.samples

import com.radixdlt.sargon.Message
import com.radixdlt.sargon.annotation.UsesSampleValues
import com.radixdlt.sargon.newMessagePlaintextSample
import com.radixdlt.sargon.newMessagePlaintextSampleOther

@UsesSampleValues
val Message.Companion.samplePlaintext: Sample<Message>
    get() = object: Sample<Message> {
        override fun invoke(): Message = newMessagePlaintextSample()

        override fun other(): Message = newMessagePlaintextSampleOther()
    }