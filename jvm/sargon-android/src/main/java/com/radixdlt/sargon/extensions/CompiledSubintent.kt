package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.BagOfBytes
import com.radixdlt.sargon.CompiledSubintent
import com.radixdlt.sargon.Subintent
import com.radixdlt.sargon.compiledSubintentBytes
import com.radixdlt.sargon.compiledSubintentDecompile

fun CompiledSubintent.decompile(): Subintent =
    compiledSubintentDecompile(compiledIntent = this)

val CompiledSubintent.bytes: BagOfBytes
    get() = compiledSubintentBytes(compiledIntent = this)