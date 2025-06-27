package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.FactorSource
import com.radixdlt.sargon.FactorSourceId
import com.radixdlt.sargon.FactorSourceIdFromAddress
import com.radixdlt.sargon.FactorSourceIdFromHash
import com.radixdlt.sargon.FactorSourceKind
import com.radixdlt.sargon.MnemonicWithPassphrase
import com.radixdlt.sargon.SpotCheckInput
import com.radixdlt.sargon.factorSourceIDFromAddressToJsonBytes
import com.radixdlt.sargon.factorSourceIDFromHashToJsonBytes
import com.radixdlt.sargon.factorSourceIDToJsonBytes
import com.radixdlt.sargon.factorSourceIdPerformSpotCheck
import com.radixdlt.sargon.factorSourcePerformSpotCheck
import com.radixdlt.sargon.newFactorSourceIDFromAddressFromJsonBytes
import com.radixdlt.sargon.newFactorSourceIDFromHashFromJsonBytes
import com.radixdlt.sargon.newFactorSourceIDFromJsonBytes
import com.radixdlt.sargon.newFactorSourceIdFromHashFromMnemonicWithPassphrase

fun FactorSourceIdFromHash.asGeneral() = FactorSourceId.Hash(value = this)

//fun FactorSourceIdFromAddress.asGeneral() = FactorSourceId.Address(
//    value = this
//)

fun FactorSourceId.Hash.Companion.init(
    kind: FactorSourceKind,
    mnemonicWithPassphrase: MnemonicWithPassphrase
) = newFactorSourceIdFromHashFromMnemonicWithPassphrase(
    factorSourceKind = kind,
    mnemonicWithPassphrase = mnemonicWithPassphrase
).asGeneral()

//@Throws(SargonException::class)
//fun FactorSourceId.Address.Companion.fromJson(
//    jsonString: String
//): FactorSourceId.Address = newFactorSourceIDFromAddressFromJsonBytes(
//    jsonBytes = bagOfBytes(fromString = jsonString)
//).asGeneral()

//fun FactorSourceId.Address.toJson(): String =
//    factorSourceIDFromAddressToJsonBytes(factorSourceIDFromAddress = value).string

@Throws(SargonException::class)
fun FactorSourceId.Hash.Companion.fromJson(
    jsonString: String
): FactorSourceId.Hash = newFactorSourceIDFromHashFromJsonBytes(
    jsonBytes = bagOfBytes(fromString = jsonString)
).asGeneral()

fun FactorSourceId.Hash.toJson(): String =
    factorSourceIDFromHashToJsonBytes(factorSourceIDFromHash = value).string

@Throws(SargonException::class)
fun FactorSourceId.Companion.fromJson(
    jsonString: String
): FactorSourceId = newFactorSourceIDFromJsonBytes(
    jsonBytes = bagOfBytes(fromString = jsonString)
)

fun FactorSourceId.toJson(): String =
    factorSourceIDToJsonBytes(factorSourceID = this).string

fun FactorSourceId.spotCheck(
    input: SpotCheckInput
) = factorSourceIdPerformSpotCheck(this, input)