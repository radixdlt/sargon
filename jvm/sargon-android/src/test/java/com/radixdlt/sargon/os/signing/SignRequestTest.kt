package com.radixdlt.sargon.os.signing

import com.radixdlt.sargon.samples.sampleAuthIntent
import com.radixdlt.sargon.samples.sampleSubintent
import com.radixdlt.sargon.samples.sampleTransactionIntent
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Assertions.assertNotEquals
import org.junit.jupiter.api.Test

class SignRequestTest {

    @Test
    fun testEqualityForTransactionIntent() {
        assertEquals(
            SignRequest.sampleTransactionIntent(),
            SignRequest.sampleTransactionIntent(),
        )
        assertEquals(
            SignRequest.sampleTransactionIntent.other(),
            SignRequest.sampleTransactionIntent.other(),
        )
    }

    @Test
    fun testInequalityForTransactionIntent() {
        assertNotEquals(
            SignRequest.sampleTransactionIntent(),
            SignRequest.sampleTransactionIntent.other(),
        )
    }

    @Test
    fun testEqualityForSubintent() {
        assertEquals(
            SignRequest.sampleSubintent(),
            SignRequest.sampleSubintent(),
        )
        assertEquals(
            SignRequest.sampleSubintent.other(),
            SignRequest.sampleSubintent.other(),
        )
    }

    @Test
    fun testInequalityForSubintent() {
        assertNotEquals(
            SignRequest.sampleSubintent(),
            SignRequest.sampleSubintent.other(),
        )
    }

    @Test
    fun testEqualityForAuthIntent() {
        assertEquals(
            SignRequest.sampleAuthIntent(),
            SignRequest.sampleAuthIntent(),
        )
        assertEquals(
            SignRequest.sampleAuthIntent.other(),
            SignRequest.sampleAuthIntent.other(),
        )
    }

    @Test
    fun testInequalityForAuthIntent() {
        assertNotEquals(
            SignRequest.sampleAuthIntent(),
            SignRequest.sampleAuthIntent.other(),
        )
    }

}