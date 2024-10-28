package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.SecurifiedU30
import com.radixdlt.sargon.U30
import com.radixdlt.sargon.bip32ConstantGlobalOffsetSecurified
import com.radixdlt.sargon.newSecurified
import com.radixdlt.sargon.newSecurifiedFromGlobalKeySpace
import com.radixdlt.sargon.newSecurifiedFromLocalKeySpace
import com.radixdlt.sargon.securifiedIndexInGlobalKeySpace
import com.radixdlt.sargon.securifiedIndexInLocalKeySpace
import kotlin.jvm.Throws

val SecurifiedU30.Companion.globalOffset: UInt
    get() = bip32ConstantGlobalOffsetSecurified()

fun SecurifiedU30.Companion.init(u30: U30): SecurifiedU30 =
    newSecurified(u30 = u30)

@Throws(SargonException::class)
fun SecurifiedU30.Companion.initFromLocal(localKeySpace: UInt): SecurifiedU30 =
    newSecurifiedFromLocalKeySpace(value = localKeySpace)

@Throws(SargonException::class)
fun SecurifiedU30.Companion.initFromGlobal(globalKeySpace: UInt): SecurifiedU30 =
    newSecurifiedFromGlobalKeySpace(value = globalKeySpace)

val SecurifiedU30.indexInLocalKeySpace: UInt
    get() = securifiedIndexInLocalKeySpace(securified = this)

val SecurifiedU30.indexInGlobalKeySpace: UInt
    get() = securifiedIndexInGlobalKeySpace(securified = this)