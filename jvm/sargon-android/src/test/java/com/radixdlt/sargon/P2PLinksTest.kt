package com.radixdlt.sargon

import com.radixdlt.sargon.extensions.P2pLinks
import com.radixdlt.sargon.extensions.asIdentifiable
import com.radixdlt.sargon.extensions.fromJson
import com.radixdlt.sargon.extensions.id
import com.radixdlt.sargon.extensions.toJson
import com.radixdlt.sargon.samples.sample
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Assertions.assertNotEquals
import org.junit.jupiter.api.Test

internal class P2PLinksTest : IdentifiedArrayTest<P2pLinks, PublicKeyHash, P2pLink>() {
    override fun element(): P2pLink = P2pLink.sample()

    override fun elementWithDifferentId(): P2pLink = P2pLink.sample.other()

    override fun identifier(element: P2pLink): PublicKeyHash = element.id

    override fun init(element: P2pLink): P2pLinks = P2pLinks(element)

    @Test
    fun testAsIdentifiable() {
        val element = element()
        val elementOther = elementWithDifferentId()
        assertEquals(
            P2pLinks(
                element,
                elementOther
            ),
            listOf(
                element,
                elementOther
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

    @Test
    fun testJsonRoundtrip() {
        val sut = listOf(element(), elementWithDifferentId())

        assertEquals(
            sut,
            P2pLinks.fromJson(json = sut.asIdentifiable().toJson())
        )
    }
}