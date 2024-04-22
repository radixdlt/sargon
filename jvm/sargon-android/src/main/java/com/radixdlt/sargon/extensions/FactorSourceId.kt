package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.BagOfBytes
import com.radixdlt.sargon.FactorSourceId
import com.radixdlt.sargon.FactorSourceIdFromAddress
import com.radixdlt.sargon.FactorSourceIdFromHash
import com.radixdlt.sargon.factorSourceIDFromAddressToJsonBytes
import com.radixdlt.sargon.factorSourceIDFromHashToJsonBytes
import com.radixdlt.sargon.newFactorSourceIDFromAddressFromJsonBytes
import com.radixdlt.sargon.newFactorSourceIDFromHashFromJsonBytes

fun FactorSourceIdFromHash.asGeneral() = FactorSourceId.Hash(value = this)

fun FactorSourceIdFromAddress.asGeneral() = FactorSourceId.Address(
    value = this
)

@Throws(SargonException::class)
fun FactorSourceId.Address.Companion.deserializeFromJsonBytes(
    jsonBytes: BagOfBytes
): FactorSourceId.Address =
    newFactorSourceIDFromAddressFromJsonBytes(jsonBytes = jsonBytes).asGeneral()

@Throws(SargonException::class)
fun FactorSourceId.Address.Companion.deserializeFromJsonString(jsonString: String): FactorSourceId.Address =
    deserializeFromJsonBytes(jsonBytes = bagOfBytes(fromString = jsonString))

fun FactorSourceId.Address.serializedJsonBytes(): BagOfBytes =
    factorSourceIDFromAddressToJsonBytes(factorSourceIDFromAddress = value)

fun FactorSourceId.Address.serializedJsonString(): String = serializedJsonBytes().string

@Throws(SargonException::class)
fun FactorSourceId.Hash.Companion.deserializeFromJsonBytes(
    jsonBytes: BagOfBytes
): FactorSourceId.Hash = newFactorSourceIDFromHashFromJsonBytes(jsonBytes = jsonBytes).asGeneral()

@Throws(SargonException::class)
fun FactorSourceId.Hash.Companion.deserializeFromJsonString(jsonString: String): FactorSourceId.Hash =
    deserializeFromJsonBytes(jsonBytes = bagOfBytes(fromString = jsonString))

fun FactorSourceId.Hash.serializedJsonBytes(): BagOfBytes =
    factorSourceIDFromHashToJsonBytes(factorSourceIDFromHash = value)

fun FactorSourceId.Hash.serializedJsonString(): String = serializedJsonBytes().string