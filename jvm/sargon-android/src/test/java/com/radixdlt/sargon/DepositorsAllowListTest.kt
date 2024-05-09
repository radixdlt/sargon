package com.radixdlt.sargon

import com.radixdlt.sargon.extensions.DepositorsAllowList
import com.radixdlt.sargon.samples.sample

internal class DepositorsAllowListTest :
    IdentifiedArrayTest<DepositorsAllowList, ResourceOrNonFungible, ResourceOrNonFungible>() {
    override fun element(): ResourceOrNonFungible = ResourceOrNonFungible.sample()

    override fun elementWithDifferentId(): ResourceOrNonFungible = ResourceOrNonFungible.sample.other()

    override fun identifier(element: ResourceOrNonFungible): ResourceOrNonFungible = element

    override fun init(element: ResourceOrNonFungible): DepositorsAllowList =
        DepositorsAllowList(element)

}