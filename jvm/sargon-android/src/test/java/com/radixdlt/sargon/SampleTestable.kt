package com.radixdlt.sargon

import com.radixdlt.sargon.samples.Sample
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Assertions.assertNotEquals
import org.junit.jupiter.api.Test

interface SampleTestable<T> {

    val samples: List<Sample<T>>

    @Test
    fun testEquality() {
        samples.forEachIndexed { index, sample ->
            assertEquals(sample(), sample(), "Sample[$index]")
            assertEquals(sample.other(), sample.other(), "Sample[$index]")
        }

    }

    @Test
    fun testInequality() {
        samples.forEachIndexed { index, sample ->
            assertNotEquals(sample(), sample.other(), "Sample[$index]")
        }
    }

    @Test
    fun testHashCode() {
        samples.forEachIndexed { index, sample ->
            assertEquals(1, setOf(sample(), sample()).size, "Sample[$index]")
            assertEquals(1, setOf(sample.other(), sample.other()).size, "Sample[$index]")
            assertEquals(2, setOf(sample(), sample.other(), sample.other(), sample()).size, "Sample[$index]")
        }
    }

}