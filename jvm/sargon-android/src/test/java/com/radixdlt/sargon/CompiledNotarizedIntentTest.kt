package com.radixdlt.sargon

import com.radixdlt.sargon.extensions.bytes
import com.radixdlt.sargon.extensions.hex
import com.radixdlt.sargon.extensions.init
import com.radixdlt.sargon.samples.Sample
import com.radixdlt.sargon.samples.sample
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Test

class CompiledNotarizedIntentTest: SampleTestable<CompiledNotarizedIntent> {
    override val samples: List<Sample<CompiledNotarizedIntent>>
        get() = listOf(CompiledNotarizedIntent.sample)

    @Test
    fun testGetData() {
        assertEquals(
            "4d22030221022104210707f20a00000000000000000a0a00000000000000090a0000002200012" +
                    "007210279be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f817980101080" +
                    "000202200202000220000202200220001210120074101ebfc1f10a3b6ed83531f16249477ab86b7" +
                    "7ce85980ef330abafbbd758caa98c665f68b8536112b6d1519feddeea01fd8429124dd75121d4bd" +
                    "88c14a27b68a123",
            CompiledNotarizedIntent.sample.other().bytes.hex
        )
    }
}