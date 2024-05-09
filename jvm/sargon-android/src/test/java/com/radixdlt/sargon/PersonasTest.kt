package com.radixdlt.sargon

import com.radixdlt.sargon.extensions.Personas
import com.radixdlt.sargon.extensions.asIdentifiable
import com.radixdlt.sargon.samples.sampleMainnet
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Assertions.assertNotEquals
import org.junit.jupiter.api.Test

internal class PersonasTest: IdentifiedArrayTest<Personas, IdentityAddress, Persona>() {
    override fun element(): Persona = Persona.sampleMainnet()

    override fun elementWithDifferentId(): Persona = Persona.sampleMainnet.other()

    override fun identifier(element: Persona): IdentityAddress = element.address

    override fun init(element: Persona): Personas = Personas(element)

    @Test
    fun testAsIdentifiable() {
        assertEquals(
            Personas(
                element(),
                elementWithDifferentId()
            ),
            listOf(
                element(),
                elementWithDifferentId()
            ).asIdentifiable()
        )
    }

    @Test
    fun testEquality() {
        val element = element()

        assertEquals(
            listOf(element).asIdentifiable(),
            listOf(element).asIdentifiable()
        )

        val collection = listOf(element).asIdentifiable()
        assertEquals(collection, collection)
        assertNotEquals(collection, "")
    }

    @Test
    fun testUniqueness() {
        val element = element()
        val elementOther = elementWithDifferentId()
        assertEquals(
            2,
            setOf(
                listOf(element).asIdentifiable(),
                listOf(elementOther).asIdentifiable(),
                listOf(element).asIdentifiable()
            ).size
        )
    }
}