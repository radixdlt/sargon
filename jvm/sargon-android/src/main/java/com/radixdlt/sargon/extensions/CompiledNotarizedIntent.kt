package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.BagOfBytes
import com.radixdlt.sargon.CompiledNotarizedIntent
import com.radixdlt.sargon.compiledNotarizedIntentGetBytes
import com.radixdlt.sargon.debugPrintCompiledNotarizedIntent
import com.radixdlt.sargon.annotation.KoverIgnore

@KoverIgnore
fun CompiledNotarizedIntent.debugPrint(): String =
    debugPrintCompiledNotarizedIntent(compiled = this)

val CompiledNotarizedIntent.bytes: BagOfBytes
    get() = compiledNotarizedIntentGetBytes(compiledNotarizedIntent = this)