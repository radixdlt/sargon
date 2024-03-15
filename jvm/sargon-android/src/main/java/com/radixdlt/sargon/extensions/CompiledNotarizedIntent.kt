package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.CompiledNotarizedIntent
import com.radixdlt.sargon.debugPrintCompiledNotarizedIntent

// TODO debug?
val CompiledNotarizedIntent.string: String
    get() = debugPrintCompiledNotarizedIntent(compiled = this)