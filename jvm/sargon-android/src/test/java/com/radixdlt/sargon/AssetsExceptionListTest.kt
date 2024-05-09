package com.radixdlt.sargon

import com.radixdlt.sargon.extensions.AssetsExceptionList
import com.radixdlt.sargon.samples.sample

internal class AssetsExceptionListTest :
    IdentifiedArrayTest<AssetsExceptionList, ResourceAddress, AssetException>() {
    override fun element(): AssetException = AssetException.sample()

    override fun elementWithDifferentId(): AssetException = AssetException.sample.other()

    override fun identifier(element: AssetException): ResourceAddress = element.address

    override fun init(element: AssetException): AssetsExceptionList = AssetsExceptionList(element)

}