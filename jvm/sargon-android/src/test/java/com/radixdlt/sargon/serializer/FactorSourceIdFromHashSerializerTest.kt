package com.radixdlt.sargon.serializer

import com.radixdlt.sargon.BagOfBytes
import com.radixdlt.sargon.FactorSourceIdFromHash
import com.radixdlt.sargon.factorSourceIDFromHashToJsonBytes
import com.radixdlt.sargon.newFactorSourceIDFromHashFromJsonBytes
import com.radixdlt.sargon.samples.sample

class FactorSourceIdFromHashSerializerTest :
    KotlinRustSerializerTest<FactorSourceIdFromHash, FactorSourceIdFromHashSerializer>(
        FactorSourceIdFromHash.sample,
        FactorSourceIdFromHashSerializer
    ) {

    override fun rustFunctionFromJsonBytes(jsonBytes: BagOfBytes): FactorSourceIdFromHash =
        newFactorSourceIDFromHashFromJsonBytes(jsonBytes)

    override fun rustFunctionToJsonBytes(value: FactorSourceIdFromHash): BagOfBytes =
        factorSourceIDFromHashToJsonBytes(value)
}