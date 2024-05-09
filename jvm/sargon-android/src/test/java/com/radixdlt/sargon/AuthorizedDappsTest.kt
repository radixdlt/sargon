package com.radixdlt.sargon

import com.radixdlt.sargon.extensions.AuthorizedDapps
import com.radixdlt.sargon.extensions.asIdentifiable
import com.radixdlt.sargon.samples.sampleMainnet
import org.junit.jupiter.api.Assertions
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Assertions.assertNotEquals
import org.junit.jupiter.api.Test

internal class AuthorizedDappsTest: IdentifiedArrayTest<AuthorizedDapps, AccountAddress, AuthorizedDapp>() {
    override fun element(): AuthorizedDapp = AuthorizedDapp.sampleMainnet()

    override fun elementWithDifferentId(): AuthorizedDapp = AuthorizedDapp.sampleMainnet.other()

    override fun identifier(element: AuthorizedDapp): AccountAddress = element.dappDefinitionAddress

    override fun init(element: AuthorizedDapp): AuthorizedDapps = AuthorizedDapps(element)

    @Test
    fun testAsIdentifiable() {
        assertEquals(
            AuthorizedDapps(
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