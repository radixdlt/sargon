package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.Bip44LikePath
import com.radixdlt.sargon.DerivationPath
import com.radixdlt.sargon.HdPathComponent
import com.radixdlt.sargon.bip44LikePathGetAddressIndex
import com.radixdlt.sargon.bip44LikePathToString
import com.radixdlt.sargon.newBip44LikePathFromIndex
import com.radixdlt.sargon.newBip44LikePathFromString
import kotlin.jvm.Throws

fun Bip44LikePath.Companion.init(index: HdPathComponent) = newBip44LikePathFromIndex(index = index)

val Bip44LikePath.displayString: String
    get() = bip44LikePathToString(path = this)

val Bip44LikePath.addressIndex: HdPathComponent
    get() = bip44LikePathGetAddressIndex(path = this)

fun Bip44LikePath.asGeneral(): DerivationPath = DerivationPath.Bip44Like(value = this)