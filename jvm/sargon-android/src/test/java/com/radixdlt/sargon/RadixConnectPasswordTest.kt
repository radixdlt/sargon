package com.radixdlt.sargon

import com.radixdlt.sargon.extensions.randomBagOfBytes
import com.radixdlt.sargon.sample.sample
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Assertions.assertNotEquals
import org.junit.jupiter.api.Test

class RadixConnectPasswordTest {

    @Test
    fun test() {
        val bytes = randomBagOfBytes(byteCount = 32)
        // test identity
        assertEquals(
            RadixConnectPassword(value = Exactly32Bytes(bagOfBytes = bytes)),
            RadixConnectPassword(value = Exactly32Bytes(bagOfBytes = bytes))
        )
        assertEquals(RadixConnectPassword.sample(), RadixConnectPassword.sample())
        assertEquals(RadixConnectPassword.sample.other(), RadixConnectPassword.sample.other())

        // inequality
        assertNotEquals(RadixConnectPassword.sample.other(), RadixConnectPassword.sample())
        assertNotEquals(RadixConnectPassword.sample(), RadixConnectPassword.sample.other())
    }

}