package com.radixdlt.sargon

import com.radixdlt.sargon.sample.sample
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Assertions.assertNotEquals
import org.junit.jupiter.api.Test

class AppPreferencesTest {

    @Test
    fun testEquals() {
        val a = AppPreferences.sample()
        val b = AppPreferences.sample.other()

        assertEquals(a, a)
        assertEquals(a, AppPreferences.sample())
        assertEquals(b, b)
        assertEquals(b, AppPreferences.sample.other())
        assertNotEquals(b, a)
        assertNotEquals(b, a)
    }

    @Test
    fun testHashcode() {
        val a = AppPreferences.sample()
        val b = AppPreferences.sample.other()
        assert(setOf(a, a).size == 1)
        assert(setOf(b, b).size == 1)
        assert(setOf(a, b, b, a).size == 2)
    }

}