package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.EntityFlag
import com.radixdlt.sargon.EntityFlags
import com.radixdlt.sargon.entityFlagsElementCount
import com.radixdlt.sargon.entityFlagsGetElements
import com.radixdlt.sargon.entityFlagsGetEntityFlagById
import com.radixdlt.sargon.newEntityFlags
import com.radixdlt.sargon.newEntityFlagsByAppending
import com.radixdlt.sargon.newEntityFlagsRemovedElement

@Throws(SargonException::class)
fun EntityFlags.Companion.init(vararg entityFlag: EntityFlag): EntityFlags =
    init(entityFlags = entityFlag.asList())

@Throws(SargonException::class)
fun EntityFlags.Companion.init(entityFlags: List<EntityFlag>): EntityFlags =
    newEntityFlags(entityFlags = entityFlags)

operator fun EntityFlags.invoke() = entityFlagsGetElements(entityFlags = this)

operator fun EntityFlags.get(index: Int) = invoke().get(index = index)

operator fun EntityFlags.contains(element: EntityFlag) = invoke().contains(element = element)

val EntityFlags.size: Int
    get() = entityFlagsElementCount(entityFlags = this).toInt()

fun EntityFlags.append(entityFlag: EntityFlag): EntityFlags =
    newEntityFlagsByAppending(entityFlag = entityFlag, to = this)

fun EntityFlags.remove(entityFlag: EntityFlag): EntityFlags =
    newEntityFlagsRemovedElement(entityFlag = entityFlag, from = this)

fun EntityFlags.get(entityFlag: EntityFlag): EntityFlag? =
    entityFlagsGetEntityFlagById(entityFlags = this, id = entityFlag)