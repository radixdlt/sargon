package com.radixdlt.sargon

import com.radixdlt.sargon.extensions.unsecuredControllingFactorInstance
import com.radixdlt.sargon.samples.Sample
import com.radixdlt.sargon.samples.sampleMainnet
import com.radixdlt.sargon.samples.sampleStokenet
import org.junit.jupiter.api.Assertions.assertNotNull
import org.junit.jupiter.api.Test

class PersonaTest: SampleTestable<Persona> {
    override val samples: List<Sample<Persona>>
        get() = listOf(Persona.sampleMainnet, Persona.sampleStokenet)

    @Test
    fun testUnsecuredControllingFactorInstance() {
        val persona = Persona.sampleMainnet()

        assertNotNull(persona.unsecuredControllingFactorInstance)
    }
}