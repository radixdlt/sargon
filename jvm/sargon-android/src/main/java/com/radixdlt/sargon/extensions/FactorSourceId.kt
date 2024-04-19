package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.FactorSourceId
import com.radixdlt.sargon.FactorSourceIdFromAddress
import com.radixdlt.sargon.FactorSourceIdFromHash

fun FactorSourceIdFromHash.asGeneral() = FactorSourceId.Hash(value = this)

fun FactorSourceIdFromAddress.asGeneral() = FactorSourceId.Address(value = this)