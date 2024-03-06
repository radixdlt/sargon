package com.radixdlt.sargon

import com.radixdlt.sargon.sample.sample
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Assertions.assertNotEquals
import org.junit.jupiter.api.Test

class HeaderTest {

    @Test
    fun testEquals() {
        val a = Header.sample()
        val b = Header.sample.other()

        assertEquals(Header.sample(), a)
        assertNotEquals(a, b)
        assertEquals(Header.sample.other(), b)
    }

    @Test
    fun testHashCode() {
        val a = Header.sample()
        val b = Header.sample.other()

        assertEquals(1, setOf(a, a).size)
        assertEquals(1, setOf(b, b).size)
        assertEquals(2, setOf(a, b, b, a).size)
    }

}