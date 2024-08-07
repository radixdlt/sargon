package com.radixdlt.sargon

import com.radixdlt.sargon.extensions.mapError
import com.radixdlt.sargon.extensions.then
import com.radixdlt.sargon.extensions.toUnit
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Assertions.assertInstanceOf
import org.junit.jupiter.api.Test
import org.junit.jupiter.api.assertThrows

class ResultTest {

    @Test
    fun testThenAfterSuccess() {
        val expected = Result.success(10)
        val plusTen = { value: Int -> Result.success(value + 10) }

        assertEquals(
            20,
            expected.then { plusTen(it) }.getOrThrow()
        )
    }

    @Test
    fun testThrowingThenAfterSuccess() {
        val expected = Result.success(10)
        val plusTen = { value: Int ->
            if (value == 10) throw RuntimeException("Some error") else Result.success(value + 10)
        }

        assertThrows<RuntimeException> {
            expected.then { plusTen(it) }.getOrThrow()
        }
    }

    @Test
    fun testThenAfterFailure() {
        val expected = Result.failure<Int>(RuntimeException("An error occurred"))
        val plusTen = { value: Int -> Result.success(value + 10) }

        assertEquals(
            expected,
            expected.then { plusTen(it) }
        )
    }

    @Test
    fun testMapErrorWhenSuccess() {
        val expected = Result.success(10)

        val result = expected.mapError { it }

        assertEquals(expected, result)
    }

    @Test
    fun testMapErrorWhenError() {
        val expected = Result.failure<Int>(RuntimeException("Some Error"))

        val result = expected.mapError { if (it is RuntimeException) IllegalStateException(it.message) else it  }

        assertThrows<IllegalStateException> {
            result.getOrThrow()
        }
    }

    @Test
    fun testToUnit() {
        val result = Result.success(10)

        assertInstanceOf(
            Unit::class.java,
            result.toUnit().getOrThrow()
        )
    }
}