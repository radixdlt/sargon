package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.IdentityAddress
import com.radixdlt.sargon.Persona
import com.radixdlt.sargon.annotation.KoverIgnore

class Personas private constructor(
    private val array: IdentifiedArray<IdentityAddress, Persona>
) : IdentifiedArray<IdentityAddress, Persona> by array {

    constructor(personas: List<Persona>) : this(
        IdentifiedArrayImpl(
            elements = personas,
            identifier = { it.address }
        )
    )

    constructor(vararg persona: Persona) : this(personas = persona.asList())

    @KoverIgnore // False positive in javaClass check
    override fun equals(other: Any?): Boolean {
        if (this === other) return true
        if (javaClass != other?.javaClass) return false

        other as Personas

        return array == other.array
    }

    override fun hashCode(): Int {
        return array.hashCode()
    }

    @KoverIgnore
    override fun toString(): String {
        return "Personas(array=$array)"
    }

}

fun List<Persona>.asIdentifiable() = Personas(personas = this)
