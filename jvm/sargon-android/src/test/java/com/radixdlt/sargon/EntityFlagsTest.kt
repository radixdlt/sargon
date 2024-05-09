package com.radixdlt.sargon

import com.radixdlt.sargon.extensions.EntityFlags
import com.radixdlt.sargon.samples.sample

internal class EntityFlagsTest: IdentifiedArrayTest<EntityFlags, EntityFlag, EntityFlag>() {
    override fun element(): EntityFlag =  EntityFlag.sample()

    override fun elementWithDifferentId(): EntityFlag =  EntityFlag.sample.other()

    override fun identifier(element: EntityFlag): EntityFlag = element

    override fun init(element: EntityFlag): EntityFlags = EntityFlags(element)

}