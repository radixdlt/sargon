package com.radixdlt.sargon

import com.radixdlt.sargon.extensions.deserializeFromJsonString
import com.radixdlt.sargon.extensions.serializedJsonString
import com.radixdlt.sargon.samples.sample
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Test

class PersonaDataEntryTest {

    @Test
    fun testEmailJsonRoundtrip() {
        val sut = PersonaDataEntryEmailAddress.sample()

        assertEquals(
            sut,
            PersonaDataEntryEmailAddress.deserializeFromJsonString(sut.serializedJsonString())
        )
    }

    @Test
    fun testPhoneJsonRoundtrip() {
        val sut = PersonaDataEntryPhoneNumber.sample()

        assertEquals(
            sut,
            PersonaDataEntryPhoneNumber.deserializeFromJsonString(sut.serializedJsonString())
        )
    }

    @Test
    fun testNameJsonRoundtrip() {
        val sut = PersonaDataEntryName.sample()

        assertEquals(
            sut,
            PersonaDataEntryName.deserializeFromJsonString(sut.serializedJsonString())
        )
    }
}