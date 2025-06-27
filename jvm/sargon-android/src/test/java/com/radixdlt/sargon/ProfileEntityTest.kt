package com.radixdlt.sargon

import com.radixdlt.sargon.extensions.ProfileEntity
import com.radixdlt.sargon.extensions.asProfileEntity
import com.radixdlt.sargon.samples.sampleMainnet
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Assertions.assertNotNull
import org.junit.jupiter.api.Test

class ProfileEntityTest {
    @Test
    fun testAccountProfileEntity() {
        val sut = Account.sampleMainnet().asProfileEntity() as ProfileEntity

        assertEquals(
            Account.sampleMainnet().networkId,
            sut.networkId
        )

        assertEquals(
            Account.sampleMainnet().address,
            (sut.address as AddressOfAccountOrPersona.Account).v1
        )

        assertEquals(
            Account.sampleMainnet().address,
            (sut as ProfileEntity.AccountEntity).accountAddress
        )

        assertEquals(
            Account.sampleMainnet().securityState,
            sut.securityState
        )

        assertEquals(
            Account.sampleMainnet().flags,
            sut.flags.asList()
        )

        assertNotNull(sut.unsecuredControllingFactorInstance)
    }

    @Test
    fun testPersonaProfileEntity() {
        val sut = Persona.sampleMainnet().asProfileEntity() as ProfileEntity

        assertEquals(
            Persona.sampleMainnet().networkId,
            sut.networkId
        )

        assertEquals(
            Persona.sampleMainnet().address,
            (sut.address as AddressOfAccountOrPersona.Identity).v1
        )

        assertEquals(
            Persona.sampleMainnet().address,
            (sut as ProfileEntity.PersonaEntity).identityAddress
        )

        assertEquals(
            Persona.sampleMainnet().securityState,
            sut.securityState
        )

        assertEquals(
            Persona.sampleMainnet().flags,
            sut.flags.asList()
        )

        assertNotNull(sut.unsecuredControllingFactorInstance)
    }
}