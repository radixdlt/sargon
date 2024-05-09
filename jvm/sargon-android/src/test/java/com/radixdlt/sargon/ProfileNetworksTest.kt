package com.radixdlt.sargon

import com.radixdlt.sargon.extensions.ProfileNetworks
import com.radixdlt.sargon.extensions.asIdentifiable
import com.radixdlt.sargon.samples.sampleMainnet
import com.radixdlt.sargon.samples.sampleStokenet
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Assertions.assertNotEquals
import org.junit.jupiter.api.Test

internal class ProfileNetworksTest: IdentifiedArrayTest<ProfileNetworks, NetworkId, ProfileNetwork>()  {
    override fun element(): ProfileNetwork = ProfileNetwork.sampleMainnet()

    override fun elementWithDifferentId(): ProfileNetwork = ProfileNetwork.sampleStokenet()

    override fun identifier(element: ProfileNetwork): NetworkId = element.id

    override fun init(element: ProfileNetwork): ProfileNetworks = ProfileNetworks(element)

    @Test
    fun testAsIdentifiable() {
        assertEquals(
            ProfileNetworks(
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