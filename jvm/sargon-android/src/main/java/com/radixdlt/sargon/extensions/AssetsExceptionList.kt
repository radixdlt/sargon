package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.AssetException
import com.radixdlt.sargon.AssetsExceptionList
import com.radixdlt.sargon.ResourceAddress
import com.radixdlt.sargon.assetsExceptionListElementCount
import com.radixdlt.sargon.assetsExceptionListGetAssetExceptionById
import com.radixdlt.sargon.assetsExceptionListGetElements
import com.radixdlt.sargon.newAssetsExceptionList
import com.radixdlt.sargon.newAssetsExceptionListByAppending
import com.radixdlt.sargon.newAssetsExceptionListByUpdatingOrAppending
import com.radixdlt.sargon.newAssetsExceptionListByUpdatingOrInsertingAtIndex
import com.radixdlt.sargon.newAssetsExceptionListRemovedById
import com.radixdlt.sargon.newAssetsExceptionListRemovedElement

fun AssetsExceptionList.Companion.init(vararg assetException: AssetException): AssetsExceptionList =
    init(assetsExceptions = assetException.asList())

fun AssetsExceptionList.Companion.init(assetsExceptions: List<AssetException>): AssetsExceptionList =
    newAssetsExceptionList(assetsExceptionList = assetsExceptions)

operator fun AssetsExceptionList.invoke() =
    assetsExceptionListGetElements(assetsExceptionList = this)

operator fun AssetsExceptionList.get(index: Int) = invoke().get(index = index)

operator fun AssetsExceptionList.contains(element: AssetException) =
    invoke().contains(element = element)

val AssetsExceptionList.size: Int
    get() = assetsExceptionListElementCount(assetsExceptionList = this).toInt()

fun AssetsExceptionList.append(assetException: AssetException): AssetsExceptionList =
    newAssetsExceptionListByAppending(assetException = assetException, to = this)

fun AssetsExceptionList.updateOrInsert(
    assetException: AssetException,
    index: Int
): AssetsExceptionList = newAssetsExceptionListByUpdatingOrInsertingAtIndex(
    assetException = assetException,
    to = this,
    index = index.toULong()
)

fun AssetsExceptionList.updateOrAppend(assetException: AssetException): AssetsExceptionList =
    newAssetsExceptionListByUpdatingOrAppending(assetException = assetException, to = this)

fun AssetsExceptionList.removeByAddress(address: ResourceAddress): AssetsExceptionList =
    newAssetsExceptionListRemovedById(idOfAssetException = address, from = this)

fun AssetsExceptionList.remove(assetException: AssetException): AssetsExceptionList =
    newAssetsExceptionListRemovedElement(assetException = assetException, from = this)

fun AssetsExceptionList.getBy(address: ResourceAddress): AssetException? =
    assetsExceptionListGetAssetExceptionById(assetsExceptionList = this, id = address)