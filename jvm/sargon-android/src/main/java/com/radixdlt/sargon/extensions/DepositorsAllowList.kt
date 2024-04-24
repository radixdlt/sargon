package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.DepositorsAllowList
import com.radixdlt.sargon.ResourceOrNonFungible
import com.radixdlt.sargon.depositorsAllowListElementCount
import com.radixdlt.sargon.depositorsAllowListGetElements
import com.radixdlt.sargon.depositorsAllowListGetResourceOrNonFungibleById
import com.radixdlt.sargon.newDepositorsAllowList
import com.radixdlt.sargon.newDepositorsAllowListByAppending
import com.radixdlt.sargon.newDepositorsAllowListByUpdatingOrAppending
import com.radixdlt.sargon.newDepositorsAllowListByUpdatingOrInsertingAtIndex
import com.radixdlt.sargon.newDepositorsAllowListRemovedElement

fun DepositorsAllowList.Companion.init(vararg resourceOrNonFungible: ResourceOrNonFungible): DepositorsAllowList =
    init(depositorsAllowList = resourceOrNonFungible.asList())

fun DepositorsAllowList.Companion.init(depositorsAllowList: List<ResourceOrNonFungible>): DepositorsAllowList =
    newDepositorsAllowList(depositorsAllowList = depositorsAllowList)

operator fun DepositorsAllowList.invoke() =
    depositorsAllowListGetElements(depositorsAllowList = this)

operator fun DepositorsAllowList.get(index: Int) = invoke().get(index = index)

operator fun DepositorsAllowList.contains(element: ResourceOrNonFungible) =
    invoke().contains(element = element)

val DepositorsAllowList.size: Int
    get() = depositorsAllowListElementCount(depositorsAllowList = this).toInt()

fun DepositorsAllowList.append(resourceOrNonFungible: ResourceOrNonFungible): DepositorsAllowList =
    newDepositorsAllowListByAppending(resourceOrNonFungible = resourceOrNonFungible, to = this)

fun DepositorsAllowList.updateOrInsert(
    resourceOrNonFungible: ResourceOrNonFungible,
    index: Int
): DepositorsAllowList = newDepositorsAllowListByUpdatingOrInsertingAtIndex(
    resourceOrNonFungible = resourceOrNonFungible,
    to = this,
    index = index.toULong()
)

fun DepositorsAllowList.updateOrAppend(
    resourceOrNonFungible: ResourceOrNonFungible
): DepositorsAllowList = newDepositorsAllowListByUpdatingOrAppending(
    resourceOrNonFungible = resourceOrNonFungible,
    to = this
)

fun DepositorsAllowList.remove(
    resourceOrNonFungible: ResourceOrNonFungible
): DepositorsAllowList = newDepositorsAllowListRemovedElement(
    resourceOrNonFungible = resourceOrNonFungible,
    from = this
)

fun DepositorsAllowList.getBy(
    resourceOrNonFungible: ResourceOrNonFungible
): ResourceOrNonFungible? = depositorsAllowListGetResourceOrNonFungibleById(
    depositorsAllowList = this,
    id = resourceOrNonFungible
)