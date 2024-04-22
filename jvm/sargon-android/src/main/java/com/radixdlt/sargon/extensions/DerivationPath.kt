package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.Bip44LikePath
import com.radixdlt.sargon.Cap26KeyKind
import com.radixdlt.sargon.Cap26Path
import com.radixdlt.sargon.DerivationPath
import com.radixdlt.sargon.GetIdPath
import com.radixdlt.sargon.HdPath
import com.radixdlt.sargon.NetworkId
import com.radixdlt.sargon.bip44LikePathGetAddressIndex
import com.radixdlt.sargon.bip44LikePathToString
import com.radixdlt.sargon.cap26PathToString
import com.radixdlt.sargon.defaultGetIdPath
import com.radixdlt.sargon.derivationPathToHdPath
import com.radixdlt.sargon.newAccountPath
import com.radixdlt.sargon.newBip44LikePathFromIndex
import com.radixdlt.sargon.newBip44LikePathFromString
import com.radixdlt.sargon.newCap26PathFromString
import com.radixdlt.sargon.newIdentityPath

typealias HDPathValue = UInt

@Throws(SargonException::class)
fun DerivationPath.Cap26.Companion.init(cap26Path: String): DerivationPath.Cap26 =
    DerivationPath.Cap26(newCap26PathFromString(string = cap26Path))

fun DerivationPath.Cap26.Companion.account(
    networkId: NetworkId,
    keyKind: Cap26KeyKind,
    index: HDPathValue
): DerivationPath.Cap26 = DerivationPath.Cap26(Cap26Path.Account(newAccountPath(
    networkId = networkId,
    keyKind = keyKind,
    index = index
)))

fun DerivationPath.Cap26.Companion.identity(
    networkId: NetworkId,
    keyKind: Cap26KeyKind,
    index: HDPathValue
): DerivationPath.Cap26 = DerivationPath.Cap26(Cap26Path.Identity(newIdentityPath(
    networkId = networkId,
    keyKind = keyKind,
    index = index
)))

val DerivationPath.Cap26.string: String
    get() = cap26PathToString(path = value)

@Throws(SargonException::class)
fun DerivationPath.Bip44Like.Companion.init(bip44LikePath: String): DerivationPath.Bip44Like =
    DerivationPath.Bip44Like(newBip44LikePathFromString(string = bip44LikePath))

fun DerivationPath.Bip44Like.Companion.init(index: HDPathValue): DerivationPath.Bip44Like =
    DerivationPath.Bip44Like(newBip44LikePathFromIndex(index = index))

val DerivationPath.Bip44Like.addressIndex: HDPathValue
    get() = bip44LikePathGetAddressIndex(path = value)

val DerivationPath.Bip44Like.string: String
    get() = bip44LikePathToString(path = value)

fun GetIdPath.Companion.default() = defaultGetIdPath()

val DerivationPath.string: String
    get() = when (this) {
        is DerivationPath.Bip44Like -> string
        is DerivationPath.Cap26 -> string
    }

val DerivationPath.hdPath: HdPath
    get() = derivationPathToHdPath(path = this)

val DerivationPath.nonHardenedIndex: HDPathValue
    get() = hdPath.components.last() // safe, we disallow empty paths.
        .nonHardenedValue

fun Bip44LikePath.asGeneral() = DerivationPath.Bip44Like(this)
fun Cap26Path.asGeneral() = DerivationPath.Cap26(this)