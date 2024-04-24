package com.radixdlt.sargon

import com.radixdlt.sargon.extensions.atLeast
import com.radixdlt.sargon.extensions.deserializeFromJsonString
import com.radixdlt.sargon.extensions.exactly
import com.radixdlt.sargon.extensions.isFulfilled
import com.radixdlt.sargon.extensions.isValid
import com.radixdlt.sargon.extensions.serializedJsonString
import com.radixdlt.sargon.samples.sample
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Assertions.assertFalse
import org.junit.jupiter.api.Assertions.assertTrue
import org.junit.jupiter.api.Test

class RequestedQuantityTest {

    @Test
    fun testExactly() {
        assertEquals(RequestedQuantity.sample(), RequestedQuantity.exactly(1))
    }

    @Test
    fun testAtLeast() {
        assertEquals(RequestedQuantity.sample.other(), RequestedQuantity.atLeast(1))
    }

    @Test
    fun testIsValid() {
        assertTrue(RequestedQuantity.sample().isValid)
    }

    @Test
    fun testIsInvalid() {
        assertFalse(RequestedQuantity.exactly(0).isValid)
    }

    @Test
    fun testIsFulfilled() {
        assertTrue(RequestedQuantity.atLeast(1).isFulfilled(1))
    }

    @Test
    fun testIsNotFulfilled() {
        assertFalse(RequestedQuantity.atLeast(2).isFulfilled(1))
        assertFalse(RequestedQuantity.exactly(2).isFulfilled(3))
    }

    @Test
    fun testJsonRoundtrip() {
        assertEquals(
            RequestedQuantity.sample(),
            RequestedQuantity.deserializeFromJsonString(
                RequestedQuantity.sample().serializedJsonString()
            )
        )
    }

}