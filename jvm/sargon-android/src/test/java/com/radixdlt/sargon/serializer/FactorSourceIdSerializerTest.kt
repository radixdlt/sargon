package com.radixdlt.sargon.serializer

import com.radixdlt.sargon.BagOfBytes
import com.radixdlt.sargon.FactorSourceId
import com.radixdlt.sargon.factorSourceIDToJsonBytes
import com.radixdlt.sargon.newFactorSourceIDFromJsonBytes
import com.radixdlt.sargon.samples.sample

class FactorSourceIdSerializerTest :
    KotlinRustSerializerTest<FactorSourceId, FactorSourceIdSerializer>(
        FactorSourceId.sample,
        FactorSourceIdSerializer
    ) {

    override fun rustFunctionFromJsonBytes(jsonBytes: BagOfBytes): FactorSourceId =
        newFactorSourceIDFromJsonBytes(jsonBytes)

    override fun rustFunctionToJsonBytes(value: FactorSourceId): BagOfBytes =
        factorSourceIDToJsonBytes(value)

}