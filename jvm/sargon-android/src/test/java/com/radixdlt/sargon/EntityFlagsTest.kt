package com.radixdlt.sargon

import com.radixdlt.sargon.extensions.EntityFlags
import com.radixdlt.sargon.extensions.asIdentifiable
import com.radixdlt.sargon.samples.sample
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Assertions.assertNotEquals
import org.junit.jupiter.api.Test

internal class EntityFlagsTest: IdentifiedArrayTest<EntityFlags, EntityFlag, EntityFlag>() {
    override fun element(): EntityFlag =  EntityFlag.sample()

    override fun elementWithDifferentId(): EntityFlag =  EntityFlag.sample.other()

    override fun identifier(element: EntityFlag): EntityFlag = element

    override fun init(element: EntityFlag): EntityFlags = EntityFlags(element)

    @Test
    fun testAsIdentifiable() {
        assertEquals(
            EntityFlags(
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