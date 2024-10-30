package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.Hardened
import com.radixdlt.sargon.HdPathComponent
import com.radixdlt.sargon.KeySpace
import com.radixdlt.sargon.hdPathComponentGetKeySpace
import com.radixdlt.sargon.hdPathComponentIndexInGlobalKeySpace
import com.radixdlt.sargon.hdPathComponentIndexInLocalKeySpace
import com.radixdlt.sargon.hdPathComponentToBip32String
import com.radixdlt.sargon.hdPathComponentToBip32StringDebug
import com.radixdlt.sargon.hdPathComponentToHardened
import com.radixdlt.sargon.newHdPathComponentFromGlobalKeySpace
import com.radixdlt.sargon.newHdPathComponentFromLocalKeySpace

fun HdPathComponent.Companion.init(globalKeySpace: UInt): HdPathComponent =
    newHdPathComponentFromGlobalKeySpace(value = globalKeySpace)

@Throws(SargonException::class)
fun HdPathComponent.Companion.init(localKeySpace: UInt, keySpace: KeySpace): HdPathComponent =
    newHdPathComponentFromLocalKeySpace(value = localKeySpace, keySpace = keySpace)

val HdPathComponent.bip32String: String
    get() = hdPathComponentToBip32String(component = this)

val HdPathComponent.keySpace: KeySpace
    get() = hdPathComponentGetKeySpace(component = this)

val HdPathComponent.indexInGlobalKeySpace: UInt
    get() = hdPathComponentIndexInGlobalKeySpace(component = this)

val HdPathComponent.indexInLocalKeySpace: UInt
    get() = hdPathComponentIndexInLocalKeySpace(component = this)

@Throws(SargonException::class)
fun HdPathComponent.asHardened(): Hardened = hdPathComponentToHardened(component = this)

