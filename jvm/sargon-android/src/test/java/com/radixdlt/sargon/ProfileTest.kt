package com.radixdlt.sargon

import com.radixdlt.sargon.extensions.init
import com.radixdlt.sargon.extensions.randomBagOfBytes
import com.radixdlt.sargon.extensions.string
import com.radixdlt.sargon.samples.sample
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Assertions.assertNotEquals
import org.junit.jupiter.api.Test

class ProfileTest {

    @Test
    fun testEquals() {
        val p = Profile.sample()
        val q = Profile.sample.other()

        assertEquals(Profile.sample(), p)
        assertEquals(p, p)
        assertEquals(q, q)
        assertEquals(Profile.sample.other(), q)
        assertNotEquals(Profile.sample(), Profile.sample.other())
    }

    @Test
    fun testHashCode() {
        val a = Profile.sample()
        val b = Profile.sample.other()

        assertEquals(1, setOf(a, a).size)
        assertEquals(1, setOf(b, b).size)
        assertEquals(2, setOf(a, b, b, a).size)
    }

    @Test
    fun testInit() {
        val hdFactorSource = PrivateHierarchicalDeterministicFactorSource.init(
            entropy = randomBagOfBytes(byteCount = 32),
            walletClientModel = WalletClientModel.ANDROID
        )

        val profile = Profile.init(
            privateHdFactorSource = hdFactorSource, creatingDeviceName = "Unit tests"
        )

        assertEquals("Unit tests - Android", profile.header.creatingDevice.description)
    }

}