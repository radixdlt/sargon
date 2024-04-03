package com.radixdlt.sargon

import com.radixdlt.sargon.extensions.plaintext
import com.radixdlt.sargon.samples.Sample
import com.radixdlt.sargon.samples.samplePlaintext
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Test

class MessageTest: SampleTestable<Message> {

    override val samples: List<Sample<Message>>
        get() = listOf(Message.samplePlaintext)

    @Test
    fun testStringRoundtrip() {
        assertEquals("Hello Radix!", Message.plaintext("Hello Radix!").plaintext)
    }

}