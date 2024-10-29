package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.U31
import com.radixdlt.sargon.newU31
import com.radixdlt.sargon.u31GetValue

@Throws(SargonException::class)
fun U31.Companion.init(value: UInt) = newU31(value = value)

val U31.value: UInt
    get() = u31GetValue(u31 = this)