package com.radixdlt.sargon

import com.radixdlt.sargon.extensions.fromJson
import com.radixdlt.sargon.extensions.toJson
import com.radixdlt.sargon.samples.sample
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Test

class PersonaDataEntryTest {

    @Test
    fun testEmailJsonRoundtrip() {
        val sut = PersonaDataEntryEmailAddress.sample()

        assertEquals(
            sut,
            PersonaDataEntryEmailAddress.fromJson(sut.toJson())
        )
    }

    @Test
    fun testPhoneJsonRoundtrip() {
        val sut = PersonaDataEntryPhoneNumber.sample()

        assertEquals(
            sut,
            PersonaDataEntryPhoneNumber.fromJson(sut.toJson())
        )
    }

    @Test
    fun testNameJsonRoundtrip() {
        val sut = PersonaDataEntryName.sample()

        assertEquals(
            sut,
            PersonaDataEntryName.fromJson(sut.toJson())
        )
    }
}