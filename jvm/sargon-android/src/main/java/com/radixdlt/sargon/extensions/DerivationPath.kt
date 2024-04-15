package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.DerivationPath
import com.radixdlt.sargon.GetIdPath
import com.radixdlt.sargon.bip44LikePathToString
import com.radixdlt.sargon.cap26PathToString
import com.radixdlt.sargon.defaultGetIdPath
import com.radixdlt.sargon.newBip44LikePathFromString
import com.radixdlt.sargon.newCap26PathFromString

@Throws(SargonException::class)
fun DerivationPath.Cap26.Companion.init(cap26Path: String): DerivationPath.Cap26 =
    DerivationPath.Cap26(newCap26PathFromString(string = cap26Path))

val DerivationPath.Cap26.string: String
    get() = cap26PathToString(path = value)

@Throws(SargonException::class)
fun DerivationPath.Bip44Like.Companion.init(bip44LikePath: String): DerivationPath.Bip44Like =
    DerivationPath.Bip44Like(newBip44LikePathFromString(string = bip44LikePath))

val DerivationPath.Bip44Like.string: String
    get() = bip44LikePathToString(path = value)

fun GetIdPath.Companion.default() = defaultGetIdPath()

val DerivationPath.string: String
    get() = when (this) {
        is DerivationPath.Bip44Like -> string
        is DerivationPath.Cap26 -> string
    }