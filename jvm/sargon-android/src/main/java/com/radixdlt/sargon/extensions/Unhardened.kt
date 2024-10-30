package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.U31
import com.radixdlt.sargon.Unhardened
import com.radixdlt.sargon.newUnhardened
import com.radixdlt.sargon.newUnhardenedFromGlobalKeySpace
import com.radixdlt.sargon.newUnhardenedFromLocalKeySpace
import com.radixdlt.sargon.unhardenedIndexInGlobalKeySpace
import com.radixdlt.sargon.unhardenedIndexInLocalKeySpace

val Unhardened.Companion.globalOffset: UInt
    get() = 0u;

fun Unhardened.Companion.init(u31: U31): Unhardened = newUnhardened(u31 = u31)

@Throws(SargonException::class)
fun Unhardened.Companion.initFromLocal(localKeySpace: UInt): Unhardened =
    newUnhardenedFromLocalKeySpace(localKeySpace)

@Throws(SargonException::class)
fun Unhardened.Companion.initFromGlobal(globalKeySpace: UInt): Unhardened =
    newUnhardenedFromGlobalKeySpace(globalKeySpace)

val Unhardened.indexInLocalKeySpace: UInt
    get() = unhardenedIndexInLocalKeySpace(unhardened = this)

val Unhardened.indexInGlobalKeySpace: UInt
    get() = unhardenedIndexInGlobalKeySpace(unhardened = this)

