package com.radixdlt.sargon

import com.radixdlt.sargon.extensions.randomBagOfBytes
import com.radixdlt.sargon.samples.sample
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Assertions.assertNotEquals
import org.junit.jupiter.api.Test

class RadixConnectPasswordTest {

    @Test
    fun test() {
        val bytes = randomBagOfBytes(byteCount = 32)
        // test identity
        assertEquals(
            RadixConnectPassword(value = newExactly32Bytes(bytes = bytes)),
            RadixConnectPassword(value = newExactly32Bytes(bytes = bytes))
        )
        assertEquals(RadixConnectPassword.sample(), RadixConnectPassword.sample())
        assertEquals(RadixConnectPassword.sample.other(), RadixConnectPassword.sample.other())

        // inequality
        assertNotEquals(RadixConnectPassword.sample.other(), RadixConnectPassword.sample())
        assertNotEquals(RadixConnectPassword.sample(), RadixConnectPassword.sample.other())
    }

}