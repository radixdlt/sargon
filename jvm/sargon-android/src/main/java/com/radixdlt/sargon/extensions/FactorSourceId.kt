package com.radixdlt.sargon.extensions

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
fun FactorSourceId.Address.Companion.fromJson(
    jsonString: String
): FactorSourceId.Address = newFactorSourceIDFromAddressFromJsonBytes(
    jsonBytes = bagOfBytes(fromString = jsonString)
).asGeneral()

fun FactorSourceId.Address.toJson(): String =
    factorSourceIDFromAddressToJsonBytes(factorSourceIDFromAddress = value).string

@Throws(SargonException::class)
fun FactorSourceId.Hash.Companion.fromJson(
    jsonString: String
): FactorSourceId.Hash = newFactorSourceIDFromHashFromJsonBytes(
    jsonBytes = bagOfBytes(fromString = jsonString)
).asGeneral()

fun FactorSourceId.Hash.toJson(): String =
    factorSourceIDFromHashToJsonBytes(factorSourceIDFromHash = value).string