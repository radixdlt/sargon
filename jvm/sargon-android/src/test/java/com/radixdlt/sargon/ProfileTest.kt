package com.radixdlt.sargon

import com.radixdlt.sargon.sample.sample
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

}