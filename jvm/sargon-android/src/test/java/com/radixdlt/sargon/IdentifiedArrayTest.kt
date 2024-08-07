package com.radixdlt.sargon

import com.radixdlt.sargon.extensions.IdentifiedArray
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Assertions.assertFalse
import org.junit.jupiter.api.Assertions.assertTrue
import org.junit.jupiter.api.Test

internal abstract class IdentifiedArrayTest<A, Identifier, Element> where A : IdentifiedArray<Identifier, Element> {

    abstract fun element(): Element

    abstract fun elementWithDifferentId(): Element

    abstract fun init(element: Element): A

    abstract fun identifier(element: Element): Identifier

    @Test
    fun testOperations() {
        val element = element()
        val elementWithDifferentId = elementWithDifferentId()

        var collection: IdentifiedArray<Identifier, Element> = init(element)

        assertEquals(1, collection.size)
        assertEquals(element, collection[0])

        collection = collection.append(elementWithDifferentId)
        assertEquals(2, collection.size)
        collection = collection.append(elementWithDifferentId) // Append again does nothing
        assertEquals(2, collection.size)
        assertEquals(elementWithDifferentId, collection[1])
        assertTrue(elementWithDifferentId in collection)

        collection = collection.remove(elementWithDifferentId)
        assertEquals(1, collection.size)
        assertFalse(elementWithDifferentId in collection)

        collection = collection.updateOrInsert(elementWithDifferentId, 1)
        assertEquals(elementWithDifferentId, collection[1])
        assertTrue(collection.size == 2)
        assertTrue(elementWithDifferentId in collection)

        collection = collection.updateOrInsert(elementWithDifferentId, 0)
        assertEquals(elementWithDifferentId, collection[1]) // The item remains in previous position
        assertTrue(collection.size == 2)

        collection = collection.updateOrAppend(elementWithDifferentId)
        assertTrue(collection.size == 2)
        collection = collection.remove(elementWithDifferentId)

        assertEquals(element, collection.getBy(identifier(element)))
        assertTrue(collection.removeBy(identifier(element)).size == 0)
    }
}