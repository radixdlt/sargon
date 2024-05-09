package com.radixdlt.sargon

import com.radixdlt.sargon.extensions.DepositorsAllowList
import com.radixdlt.sargon.extensions.asIdentifiable
import com.radixdlt.sargon.samples.sample
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Assertions.assertNotEquals
import org.junit.jupiter.api.Test

internal class DepositorsAllowListTest :
    IdentifiedArrayTest<DepositorsAllowList, ResourceOrNonFungible, ResourceOrNonFungible>() {
    override fun element(): ResourceOrNonFungible = ResourceOrNonFungible.sample()

    override fun elementWithDifferentId(): ResourceOrNonFungible = ResourceOrNonFungible.sample.other()

    override fun identifier(element: ResourceOrNonFungible): ResourceOrNonFungible = element

    override fun init(element: ResourceOrNonFungible): DepositorsAllowList =
        DepositorsAllowList(element)

    @Test
    fun testAsIdentifiable() {
        assertEquals(
            DepositorsAllowList(
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