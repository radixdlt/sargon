package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.CompiledNotarizedIntent
import com.radixdlt.sargon.debugPrintCompiledNotarizedIntent
import com.radixdlt.sargon.utils.KoverIgnore

@KoverIgnore
fun CompiledNotarizedIntent.debugPrint(): String =
    debugPrintCompiledNotarizedIntent(compiled = this)