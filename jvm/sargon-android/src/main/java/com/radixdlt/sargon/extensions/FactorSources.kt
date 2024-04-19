package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.DeviceFactorSource
import com.radixdlt.sargon.FactorSource
import com.radixdlt.sargon.FactorSourceId
import com.radixdlt.sargon.FactorSources
import com.radixdlt.sargon.LedgerHardwareWalletFactorSource
import com.radixdlt.sargon.factorSourcesElementCount
import com.radixdlt.sargon.factorSourcesGetElements
import com.radixdlt.sargon.factorSourcesGetFactorSourceById
import com.radixdlt.sargon.newFactorSources
import com.radixdlt.sargon.newFactorSourcesByAppending
import com.radixdlt.sargon.newFactorSourcesRemovedById
import com.radixdlt.sargon.newFactorSourcesRemovedElement

@Throws(SargonException::class)
fun FactorSources.Companion.init(vararg factorSource: FactorSource): FactorSources =
    init(factorSources = factorSource.asList())

@Throws(SargonException::class)
fun FactorSources.Companion.init(factorSources: List<FactorSource>): FactorSources =
    newFactorSources(factorSources = factorSources)

operator fun FactorSources.invoke() = factorSourcesGetElements(factorSources = this)

operator fun FactorSources.get(index: Int) = invoke().get(index = index)

operator fun FactorSources.contains(element: FactorSource) = invoke().contains(element = element)

val FactorSources.size: Int
    get() = factorSourcesElementCount(factorSources = this).toInt()

fun FactorSources.append(factorSource: FactorSource): FactorSources =
    newFactorSourcesByAppending(factorSource = factorSource, to = this)

/**
 * FactorSources is NonEmpty, so this throws if the resulting collection would be empty when
 * removing the element would result in an empty copy.
 */
@Throws(SargonException::class)
fun FactorSources.remove(factorSource: FactorSource): FactorSources =
    newFactorSourcesRemovedElement(factorSource = factorSource, from = this)

/**
 * FactorSources is NonEmpty, so this throws if the resulting collection would be empty when
 * removing the element  by id would result in an empty copy.
 */
@Throws(SargonException::class)
fun FactorSources.removeById(id: FactorSourceId): FactorSources =
    newFactorSourcesRemovedById(idOfFactorSource = id, from = this)

fun FactorSources.get(id: FactorSourceId): FactorSource? =
    factorSourcesGetFactorSourceById(factorSources = this, id = id)

fun DeviceFactorSource.asGeneral() = FactorSource.Device(this)
fun LedgerHardwareWalletFactorSource.asGeneral() = FactorSource.Ledger(this)

