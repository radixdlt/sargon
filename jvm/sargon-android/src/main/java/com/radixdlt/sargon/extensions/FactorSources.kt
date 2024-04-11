package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.FactorSource
import com.radixdlt.sargon.FactorSources
import com.radixdlt.sargon.getFactorSources
import com.radixdlt.sargon.newFactorSources

@Throws(SargonException::class)
fun FactorSources.Companion.init(vararg factorSource: FactorSource): FactorSources =
    newFactorSources(factorSources = factorSource.asList())

@Throws(SargonException::class)
fun FactorSources.Companion.init(factorSources: List<FactorSource>): FactorSources =
    newFactorSources(factorSources = factorSources)

operator fun FactorSources.invoke() = getFactorSources(factorSources = this)

operator fun FactorSources.get(index: Int) = invoke().get(index = index)

operator fun FactorSources.contains(element: FactorSource) = invoke().contains(element = element)

val FactorSources.size: Int
    get() = invoke().size