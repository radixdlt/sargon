package com.radixdlt.sargon

import com.radixdlt.sargon.extensions.Personas
import com.radixdlt.sargon.samples.sampleMainnet

internal class PersonasTest: IdentifiedArrayTest<Personas, IdentityAddress, Persona>() {
    override fun element(): Persona = Persona.sampleMainnet()

    override fun elementWithDifferentId(): Persona = Persona.sampleMainnet.other()

    override fun identifier(element: Persona): IdentityAddress = element.address

    override fun init(element: Persona): Personas = Personas(element)

}