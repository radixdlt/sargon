package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.U30
import com.radixdlt.sargon.newU30
import com.radixdlt.sargon.u30GetValue
import kotlin.jvm.Throws

@Throws(SargonException::class)
fun U30.Companion.init(value: UInt) = newU30(value = value)

val U30.value: UInt
    get() = u30GetValue(u30 = this)