package com.radixdlt.sargon

import com.radixdlt.sargon.extensions.init
import com.radixdlt.sargon.extensions.randomBagOfBytes
import com.radixdlt.sargon.samples.Sample
import com.radixdlt.sargon.samples.sample
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Assertions.assertNotEquals
import org.junit.jupiter.api.Test

class RadixConnectPasswordTest: SampleTestable<RadixConnectPassword> {

    override val samples: List<Sample<RadixConnectPassword>>
        get() = listOf(RadixConnectPassword.sample)

    @Test
    fun test() {
        val bytes = randomBagOfBytes(byteCount = 32)
        // test identity
        assertEquals(
            RadixConnectPassword.init(bytes = Exactly32Bytes.init(bytes = bytes)),
            RadixConnectPassword.init(bytes = Exactly32Bytes.init(bytes = bytes))
        )
        assertEquals(RadixConnectPassword.sample(), RadixConnectPassword.sample())
        assertEquals(RadixConnectPassword.sample.other(), RadixConnectPassword.sample.other())

        // inequality
        assertNotEquals(RadixConnectPassword.sample.other(), RadixConnectPassword.sample())
        assertNotEquals(RadixConnectPassword.sample(), RadixConnectPassword.sample.other())
    }

}