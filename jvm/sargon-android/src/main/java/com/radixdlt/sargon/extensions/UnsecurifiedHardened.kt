package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.U30
import com.radixdlt.sargon.UnsecurifiedHardened
import com.radixdlt.sargon.bip32ConstantGlobalOffsetHardened
import com.radixdlt.sargon.newUnsecurifiedHardened
import com.radixdlt.sargon.newUnsecurifiedHardenedFromGlobalKeySpace
import com.radixdlt.sargon.newUnsecurifiedHardenedFromLocalKeySpace
import com.radixdlt.sargon.unsecurifiedHardenedIndexInGlobalKeySpace
import com.radixdlt.sargon.unsecurifiedHardenedIndexInLocalKeySpace

val UnsecurifiedHardened.Companion.globalOffset: UInt
    get() = bip32ConstantGlobalOffsetHardened()

fun UnsecurifiedHardened.Companion.init(u30: U30): UnsecurifiedHardened =
    newUnsecurifiedHardened(u30 = u30)

fun UnsecurifiedHardened.Companion.initFromLocal(localKeySpace: UInt): UnsecurifiedHardened =
    newUnsecurifiedHardenedFromLocalKeySpace(value = localKeySpace)

fun UnsecurifiedHardened.Companion.initFromGlobal(globalKeySpace: UInt): UnsecurifiedHardened =
    newUnsecurifiedHardenedFromGlobalKeySpace(value = globalKeySpace)

val UnsecurifiedHardened.indexInLocalKeySpace: UInt
    get() = unsecurifiedHardenedIndexInLocalKeySpace(unsecurifiedHardened = this)

val UnsecurifiedHardened.indexInGlobalKeySpace: UInt
    get() = unsecurifiedHardenedIndexInGlobalKeySpace(unsecurifiedHardened = this)