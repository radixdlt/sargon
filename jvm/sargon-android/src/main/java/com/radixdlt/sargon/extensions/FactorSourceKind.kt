package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.FactorSourceKind
import com.radixdlt.sargon.factorSourceKindToString
import com.radixdlt.sargon.newFactorSourceKindFromString

@Throws(SargonException::class)
fun FactorSourceKind.Companion.init(string: String) = newFactorSourceKindFromString(string = string)

val FactorSourceKind.string: String
    get() = factorSourceKindToString(kind = this)