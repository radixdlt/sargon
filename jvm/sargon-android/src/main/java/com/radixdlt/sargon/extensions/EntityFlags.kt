package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.EntityFlag
import com.radixdlt.sargon.annotation.KoverIgnore

class EntityFlags private constructor(
    private val array: IdentifiedArray<EntityFlag, EntityFlag>
) : IdentifiedArray<EntityFlag, EntityFlag> by array {

    constructor(entityFlags: List<EntityFlag>) : this(
        IdentifiedArrayImpl(
            elements = entityFlags,
            identifier = { it }
        )
    )

    constructor(vararg entityFlag: EntityFlag) : this(entityFlags = entityFlag.asList())

    @KoverIgnore // False positive in javaClass check
    override fun equals(other: Any?): Boolean {
        if (this === other) return true
        if (javaClass != other?.javaClass) return false

        other as EntityFlags

        return array == other.array
    }

    override fun hashCode(): Int {
        return array.hashCode()
    }

    @KoverIgnore
    override fun toString(): String {
        return "EntityFlags(array=$array)"
    }

}

fun List<EntityFlag>.asIdentifiable() = EntityFlags(entityFlags = this)
