package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.IdentityAddress
import com.radixdlt.sargon.Persona

class Personas private constructor(
    array: IdentifiedArray<IdentityAddress, Persona>
) : IdentifiedArray<IdentityAddress, Persona> by array {

    constructor(personas: List<Persona>) : this(
        IdentifiedArrayImpl(
            elements = personas,
            identifier = { it.address }
        )
    )

    constructor(vararg persona: Persona) : this(personas = persona.asList())
}