package com.radixdlt.sargon.serializer

import com.radixdlt.sargon.BagOfBytes
import com.radixdlt.sargon.FactorSourceIdFromHash
import com.radixdlt.sargon.factorSourceIDFromHashToJsonBytes
import com.radixdlt.sargon.newFactorSourceIDFromHashFromJsonBytes

object FactorSourceIdFromHashSerializer: KotlinRustSerializer<FactorSourceIdFromHash>(
    serialName = "com.radixdlt.sargon.FactorSourceIdFromHash"
) {
    override fun rustFunctionFromJsonBytes(jsonBytes: BagOfBytes): FactorSourceIdFromHash =
        newFactorSourceIDFromHashFromJsonBytes(jsonBytes = jsonBytes)

    override fun rustFunctionToJsonBytes(value: FactorSourceIdFromHash): BagOfBytes =
        factorSourceIDFromHashToJsonBytes(factorSourceIDFromHash = value)
}