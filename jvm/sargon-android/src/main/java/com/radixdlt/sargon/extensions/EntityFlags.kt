package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.EntityFlag

class EntityFlags private constructor(
    array: IdentifiedArray<EntityFlag, EntityFlag>
) : IdentifiedArray<EntityFlag, EntityFlag> by array {

    constructor(entityFlags: List<EntityFlag>) : this(
        IdentifiedArrayImpl(
            elements = entityFlags,
            identifier = { it }
        )
    )

    constructor(vararg entityFlag: EntityFlag) : this(entityFlags = entityFlag.asList())

    companion object
}