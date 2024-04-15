package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.Persona
import com.radixdlt.sargon.Personas
import com.radixdlt.sargon.newPersonas
import com.radixdlt.sargon.personasGetElements

fun Personas.Companion.init(vararg persona: Persona): Personas =
    newPersonas(personas = persona.asList())

fun Personas.Companion.init(personas: List<Persona>): Personas = newPersonas(personas = personas)

operator fun Personas.invoke() = personasGetElements(personas = this)

operator fun Personas.get(index: Int) = invoke().get(index = index)

operator fun Personas.contains(element: Persona) = invoke().contains(element = element)

val Personas.size: Int
    get() = invoke().size