package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.IdentityAddress
import com.radixdlt.sargon.P2pLink
import com.radixdlt.sargon.P2pLinks
import com.radixdlt.sargon.Persona
import com.radixdlt.sargon.Personas
import com.radixdlt.sargon.newP2PLinksByUpdatingOrAppending
import com.radixdlt.sargon.newP2PLinksByUpdatingOrInsertingAtIndex
import com.radixdlt.sargon.newPersonas
import com.radixdlt.sargon.newPersonasByAppending
import com.radixdlt.sargon.newPersonasByUpdatingOrAppending
import com.radixdlt.sargon.newPersonasByUpdatingOrInsertingAtIndex
import com.radixdlt.sargon.newPersonasRemovedById
import com.radixdlt.sargon.newPersonasRemovedElement
import com.radixdlt.sargon.personasElementCount
import com.radixdlt.sargon.personasGetElements
import com.radixdlt.sargon.personasGetPersonaById

fun Personas.Companion.init(vararg persona: Persona): Personas = init(personas = persona.asList())

fun Personas.Companion.init(personas: List<Persona>): Personas = newPersonas(personas = personas)

operator fun Personas.invoke() = personasGetElements(personas = this)

operator fun Personas.get(index: Int) = invoke().get(index = index)

operator fun Personas.contains(element: Persona) = invoke().contains(element = element)

val Personas.size: Int
    get() = personasElementCount(personas = this).toInt()

fun Personas.append(persona: Persona): Personas =
    newPersonasByAppending(persona = persona, to = this)

fun Personas.updateOrInsert(persona: Persona, index: Int): Personas =
    newPersonasByUpdatingOrInsertingAtIndex(
        persona = persona,
        to = this,
        index = index.toULong()
    )

fun Personas.updateOrAppend(persona: Persona): Personas =
    newPersonasByUpdatingOrAppending(persona = persona, to = this)

fun Personas.removeByAddress(address: IdentityAddress): Personas =
    newPersonasRemovedById(idOfPersona = address, from = this)

fun Personas.remove(persona: Persona): Personas =
    newPersonasRemovedElement(persona = persona, from = this)

fun Personas.getBy(address: IdentityAddress): Persona? =
    personasGetPersonaById(personas = this, id = address)