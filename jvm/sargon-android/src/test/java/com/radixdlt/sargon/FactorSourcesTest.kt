package com.radixdlt.sargon

import com.radixdlt.sargon.sample.factorSourcesSample
import com.radixdlt.sargon.sample.factorSourcesSampleOther
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Assertions.assertNotEquals
import org.junit.jupiter.api.Test

class FactorSourcesTest {

    @Test
    fun testEquals() {
        val a = factorSourcesSample()
        val b = factorSourcesSampleOther()

        assertEquals(a, a)
        assertEquals(b, b)
        assertNotEquals(b, a)
        assertNotEquals(b, a)
    }

    @Test
    fun testHashCode() {
        val a = factorSourcesSample()
        val b = factorSourcesSampleOther()
        assertEquals(1, setOf(a, a).size)
        assertEquals(1, setOf(b, b).size)
        assertEquals(2, setOf(a, b, b, a).size)
    }

}