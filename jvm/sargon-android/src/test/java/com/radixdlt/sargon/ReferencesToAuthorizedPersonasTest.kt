package com.radixdlt.sargon

import com.radixdlt.sargon.extensions.ReferencesToAuthorizedPersonas
import com.radixdlt.sargon.extensions.asIdentifiable
import com.radixdlt.sargon.samples.sampleMainnet
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Assertions.assertNotEquals
import org.junit.jupiter.api.Test

internal class ReferencesToAuthorizedPersonasTest :
    IdentifiedArrayTest<ReferencesToAuthorizedPersonas, IdentityAddress, AuthorizedPersonaSimple>() {
    override fun element(): AuthorizedPersonaSimple = AuthorizedPersonaSimple.sampleMainnet()

    override fun elementWithDifferentId(): AuthorizedPersonaSimple = AuthorizedPersonaSimple.sampleMainnet.other()

    override fun identifier(element: AuthorizedPersonaSimple): IdentityAddress =
        element.identityAddress

    override fun init(element: AuthorizedPersonaSimple): ReferencesToAuthorizedPersonas =
        ReferencesToAuthorizedPersonas(element)

    @Test
    fun testAsIdentifiable() {
        assertEquals(
            ReferencesToAuthorizedPersonas(
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