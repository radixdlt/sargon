package com.radixdlt.sargon

import com.radixdlt.sargon.extensions.MAX
import com.radixdlt.sargon.extensions.MIN
import com.radixdlt.sargon.extensions.abs
import com.radixdlt.sargon.extensions.ceil
import com.radixdlt.sargon.extensions.clamped
import com.radixdlt.sargon.extensions.compareTo
import com.radixdlt.sargon.extensions.div
import com.radixdlt.sargon.extensions.exponent
import com.radixdlt.sargon.extensions.floor
import com.radixdlt.sargon.extensions.formatted
import com.radixdlt.sargon.extensions.formattedPlain
import com.radixdlt.sargon.extensions.init
import com.radixdlt.sargon.extensions.isNegative
import com.radixdlt.sargon.extensions.isPositive
import com.radixdlt.sargon.extensions.negative
import com.radixdlt.sargon.extensions.orZero
import com.radixdlt.sargon.extensions.plus
import com.radixdlt.sargon.extensions.rounded
import com.radixdlt.sargon.extensions.string
import com.radixdlt.sargon.extensions.sumOf
import com.radixdlt.sargon.extensions.times
import com.radixdlt.sargon.extensions.toDecimal192
import com.radixdlt.sargon.extensions.toDecimal192OrNull
import com.radixdlt.sargon.samples.Sample
import com.radixdlt.sargon.samples.sample
import io.mockk.every
import io.mockk.mockk
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Assertions.assertFalse
import org.junit.jupiter.api.Assertions.assertNotNull
import org.junit.jupiter.api.Assertions.assertNull
import org.junit.jupiter.api.Assertions.assertTrue
import org.junit.jupiter.api.Test
import org.junit.jupiter.api.assertDoesNotThrow
import org.junit.jupiter.api.assertThrows
import java.text.DecimalFormat
import java.text.DecimalFormatSymbols
import java.util.Locale
import kotlin.math.PI

class Decimal192Test : SampleTestable<Decimal192> {

    override val samples: List<Sample<Decimal192>>
        get() = listOf(Decimal192.sample)

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
        assertEquals(1.5f.toDecimal192(), three * one / two)
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
        assertEquals("1.23", a.rounded(decimalPlaces = 2u).string)
        assertEquals("1.23", a.floor(decimalPlaces = 2u).string)
        assertEquals("1.24", a.ceil(decimalPlaces = 2u).string)
        assertEquals("0", (a * (-1).toDecimal192()).clamped.string)
        assertEquals(a.string, a.clamped.string)

        assertThrows<IllegalArgumentException> {
            a.rounded(decimalPlaces = UByte.MAX_VALUE)
        }

        assertThrows<IllegalStateException> {
            Decimal192.MAX.rounded(
                decimalPlaces = 0.toUByte(),
                roundingMode = RoundingMode.AWAY_FROM_ZERO
            )
        }
    }

    @Test
    fun testBoundaries() {
        assertEquals(
            "3138550867693340381917894711603833208051.177722232017256447",
            Decimal192.MAX.plainString()
        )
        assertEquals(
            "-3138550867693340381917894711603833208051.177722232017256447",
            Decimal192.MIN.plainString()
        )
    }

    @Test
    fun testExponent() {
        assertEquals("100", Decimal192.exponent(exponent = 2.toUByte()).plainString())
        assertEquals("1000", Decimal192.exponent(exponent = 3.toUByte()).plainString())
    }

    @Test
    fun testNegativePositive() {
        val positive = Decimal192.exponent(2.toUByte())
        assertTrue(positive.isPositive)
        assertFalse(positive.isNegative)
        assertTrue(positive.negative().isNegative)
        assertTrue(positive.negative().abs().isPositive)
    }

    @Test
    fun testConversions() {
        assertEquals("100.2", "100.2".toDecimal192().string)
        assertThrows<CommonException.DecimalException> {
            "100,2".toDecimal192()
        }
        assertEquals(Long.MAX_VALUE.toString(), Long.MAX_VALUE.toDecimal192().string)
        assertEquals(Long.MIN_VALUE.toString(), Long.MIN_VALUE.toDecimal192().string)
        assertEquals(Int.MAX_VALUE.toString(), Int.MAX_VALUE.toDecimal192().string)
        assertEquals(Int.MIN_VALUE.toString(), Int.MIN_VALUE.toDecimal192().string)
        assertEquals(ULong.MAX_VALUE.toString(), ULong.MAX_VALUE.toDecimal192().string)
        assertEquals(ULong.MIN_VALUE.toString(), ULong.MIN_VALUE.toDecimal192().string)
        assertEquals(UInt.MAX_VALUE.toString(), UInt.MAX_VALUE.toDecimal192().string)
        assertEquals(UInt.MIN_VALUE.toString(), UInt.MIN_VALUE.toDecimal192().string)
        assertEquals(
            Float.MAX_VALUE.toBigDecimal().toPlainString(),
            Float.MAX_VALUE.toDecimal192().plainString()
        )
        assertThrows<CommonException.DecimalOverflow> {
            Float.MIN_VALUE.toDecimal192()
        }
    }

    @Test
    fun testStringParsing() {
        val usFormat = DecimalFormat().apply {
            decimalFormatSymbols = DecimalFormatSymbols(Locale.US)
        }
        val greekFormat = DecimalFormat().apply {
            decimalFormatSymbols = DecimalFormatSymbols(Locale.forLanguageTag("el"))
        }

        assertEquals(
            Decimal192.init(usFormat.format(PI), DecimalFormatSymbols(Locale.US)),
            Decimal192.init(
                greekFormat.format(PI),
                DecimalFormatSymbols(Locale.forLanguageTag("el"))
            )
        )

        assertThrows<CommonException.DecimalException> {
            greekFormat.format(PI).toDecimal192()
        }

        assertDoesNotThrow {
            Decimal192.init(DecimalFormat().format(PI))
        }
    }

    @Test
    fun testFromDouble() {
        assertEquals("0.1", 0.1.toDecimal192().string)
        assertEquals("4.012345678901235", 4.012345678901234567895555555.toDecimal192().string)
    }

    @Test
    fun testOrNull() {
        assertNull(Double.MAX_VALUE.toDecimal192OrNull())
        assertNotNull(10.0.toDecimal192OrNull())

        assertNull(Float.MIN_VALUE.toDecimal192OrNull())
        assertNotNull(3.14f.toDecimal192OrNull())

        assertNull(Float.MIN_VALUE.toString().toDecimal192OrNull())
        assertNotNull("3.14".toDecimal192OrNull())
    }

    @Test
    fun testOrZero() {
        val value: Decimal192? = null
        assertEquals(0.toDecimal192(), value.orZero())
        assertEquals(10.toDecimal192(), 10.toDecimal192().orZero())
    }

    @Test
    fun testFormatting() {
        val decimalFormatSymbols = mockk<DecimalFormatSymbols>(relaxed = true).apply {
            every { decimalSeparator } returns ','
            every { groupingSeparator } returns ' '
        }

        val sut = "1013.1415".toDecimal192()

        assertEquals(
            "1 013,14", sut.formatted(
                format = decimalFormatSymbols,
                totalPlaces = 6.toUByte(),
                useGroupingSeparator = true
            )
        )
    }

    @Test
    fun testSum() {
        val items = listOf(
            0.toDecimal192(),
            10.toDecimal192(),
            20.toDecimal192()
        )

        assertEquals(30.toDecimal192(), items.sumOf { it })
    }

    private fun Decimal192.plainString() = formattedPlain(
        format = DecimalFormatSymbols.getInstance(Locale.US),
        useGroupingSeparator = false
    )
}