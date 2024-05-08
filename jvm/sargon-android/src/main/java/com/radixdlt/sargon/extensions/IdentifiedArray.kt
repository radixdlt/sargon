package com.radixdlt.sargon.extensions

internal interface IdentifiedArray<Identifier, Element> {
    val size: Int

    fun asList(): List<Element>

    fun append(element: Element): IdentifiedArray<Identifier, Element>

    fun updateOrAppend(element: Element): IdentifiedArray<Identifier, Element>

    fun updateOrInsert(element: Element, index: Int): IdentifiedArray<Identifier, Element>

    fun remove(element: Element): IdentifiedArray<Identifier, Element>

    fun getBy(identifier: Identifier): Element?

    operator fun get(index: Int): Element

    fun removeBy(identifier: Identifier): IdentifiedArray<Identifier, Element>
}


internal class IdentifiedArrayImpl<Identifier, Element>(
    elements: List<Element>,
    private val identifier: (Element) -> Identifier
): IdentifiedArray<Identifier, Element> {

    private val inner: LinkedHashMap<Identifier, Element> = LinkedHashMap(
        LinkedHashMap<Identifier, Element>().apply { elements.forEach { this[identifier(it)] = it } }
    )

    override val size: Int
        get() = inner.size

    override fun asList(): List<Element> = inner
        .values
        .toList()

    override fun get(index: Int): Element = inner.values.elementAt(index)

    override fun append(element: Element) = apply {
        val identifier = identifier(element)
        if (!inner.contains(identifier)) {
            inner[identifier] = element
        }
    }

    override fun updateOrAppend(element: Element) = apply {
        inner[identifier(element)] = element
    }

    override fun updateOrInsert(element: Element, index: Int): IdentifiedArray<Identifier, Element> {
        val id = identifier(element)

        return if (inner.contains(id)) {
            inner[id] = element

            this
        } else {
            val list = inner.values.toMutableList()
            list.add(index, element)

            IdentifiedArrayImpl(elements = list.toList(), identifier = identifier)
        }
    }

    override fun remove(element: Element) = apply {
        val key = identifier(element)

        inner.remove(key)
    }

    override fun getBy(identifier: Identifier) = inner[identifier]

    override fun removeBy(identifier: Identifier) = apply {
        inner.remove(identifier)
    }
}