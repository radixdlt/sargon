package com.radixdlt.sargon

import com.radixdlt.sargon.extensions.ceil
import com.radixdlt.sargon.extensions.clamped
import com.radixdlt.sargon.extensions.compareTo
import com.radixdlt.sargon.extensions.floor
import com.radixdlt.sargon.extensions.plus
import com.radixdlt.sargon.extensions.rounded
import com.radixdlt.sargon.extensions.string
import com.radixdlt.sargon.extensions.times
import com.radixdlt.sargon.extensions.toDecimal192
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Assertions.assertTrue
import org.junit.jupiter.api.Test

class Decimal192Test {

    @Test
    fun test() {
        val one = 1.toDecimal192()
        val two = 2.toDecimal192()
        val three = 3.toDecimal192()

        assertEquals(three, one + two)

        val a = "958947355801916604025588861116008628224.01234".toDecimal192()
        val b = "58947355801916604025588861116008628224.04321".toDecimal192()
        val c = "1017894711603833208051177722232017256448.05555".toDecimal192()
        assertEquals(c, a + b)

        val d = 0.000000000000000123f.toDecimal192()
        val e = 0.000000000000000321f.toDecimal192()
        val f = 0.000000000000000444f.toDecimal192()

        assertEquals("0.000000000000000123", d.string)
        assertEquals(f, d + e)
    }

    @Test
    fun testComparable() {
        val one = 1.toDecimal192()
        val two = 2.toDecimal192()
        val three = 3.toDecimal192()

        assertTrue(one < two)
        assertTrue(three > two)
        assertTrue(three <= three)
        assertTrue(three == 3.toDecimal192())
    }

    @Test
    fun testRounding() {
        val a = 1.233445f.toDecimal192()
        assertEquals("1.23", a.rounded(decimalPlaces = 2).string)
        assertEquals("1.23", a.floor(decimalPlaces = 2).string)
        assertEquals("1.24", a.ceil(decimalPlaces = 2).string)
        assertEquals("0", (a * (-1).toDecimal192()).clamped.string)
        assertEquals(a.string, a.clamped.string)
    }
}