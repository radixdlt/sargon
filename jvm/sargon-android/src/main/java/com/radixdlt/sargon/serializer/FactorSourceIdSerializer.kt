package com.radixdlt.sargon.serializer

import com.radixdlt.sargon.BagOfBytes
import com.radixdlt.sargon.FactorSourceId
import com.radixdlt.sargon.factorSourceIDToJsonBytes
import com.radixdlt.sargon.newFactorSourceIDFromJsonBytes

object FactorSourceIdSerializer: KotlinRustSerializer<FactorSourceId>(
    serialName = "com.radixdlt.sargon.FactorSourceId"
) {
    override fun rustFunctionFromJsonBytes(jsonBytes: BagOfBytes): FactorSourceId =
        newFactorSourceIDFromJsonBytes(jsonBytes = jsonBytes)

    override fun rustFunctionToJsonBytes(value: FactorSourceId): BagOfBytes =
        factorSourceIDToJsonBytes(factorSourceID = value)
}