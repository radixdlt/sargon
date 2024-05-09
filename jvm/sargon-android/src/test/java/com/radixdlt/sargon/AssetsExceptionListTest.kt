package com.radixdlt.sargon

import com.radixdlt.sargon.extensions.AssetsExceptionList
import com.radixdlt.sargon.extensions.asIdentifiable
import com.radixdlt.sargon.samples.sample
import org.junit.jupiter.api.Assertions
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Assertions.assertNotEquals
import org.junit.jupiter.api.Test

internal class AssetsExceptionListTest :
    IdentifiedArrayTest<AssetsExceptionList, ResourceAddress, AssetException>() {
    override fun element(): AssetException = AssetException.sample()

    override fun elementWithDifferentId(): AssetException = AssetException.sample.other()

    override fun identifier(element: AssetException): ResourceAddress = element.address

    override fun init(element: AssetException): AssetsExceptionList = AssetsExceptionList(element)

    @Test
    fun testAsIdentifiable() {
        assertEquals(
            AssetsExceptionList(
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