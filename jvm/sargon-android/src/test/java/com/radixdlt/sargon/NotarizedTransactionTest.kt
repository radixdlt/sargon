package com.radixdlt.sargon

import com.radixdlt.sargon.extensions.bytes
import com.radixdlt.sargon.extensions.compile
import com.radixdlt.sargon.extensions.hex
import com.radixdlt.sargon.extensions.init
import com.radixdlt.sargon.samples.Sample
import com.radixdlt.sargon.samples.sample
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Test

class NotarizedTransactionTest: SampleTestable<NotarizedTransaction> {
    override val samples: List<Sample<NotarizedTransaction>>
        get() = listOf(NotarizedTransaction.sample)

    @Test
    fun testGetData() {
        assertEquals(
            "4d22030221022104210707010a872c0100000000000a912c01000000000009092f24002201012" +
                    "00720ec172b93ad5e563bf4932c70e1245034c35467ef2efd4d64ebf819683467e2bf010108000" +
                    "020220441038000d1be9c042f627d98a01383987916d43cf439631ca1d8c8076d6754ab263d0c0" +
                    "86c6f636b5f6665652101850000fda0c4277708000000000000000000000000000000004103800" +
                    "0d1be9c042f627d98a01383987916d43cf439631ca1d8c8076d6754ab263d0c087769746864726" +
                    "177210280005da66318c6318c61f5a61b4c6318c6318cf794aa8d295f14e6318c6318c68500004" +
                    "43945309a7a48000000000000000000000000000000000280005da66318c6318c61f5a61b4c631" +
                    "8c6318cf794aa8d295f14e6318c6318c6850000443945309a7a480000000000000000000000000" +
                    "0000041038000d1127918c16af09af521951adcf3a20ab2cc87c0e72e85814764853ce5e70c147" +
                    "472795f6465706f7369745f6f725f61626f72742102810000000022000020200022010121020c0" +
                    "a746578742f706c61696e2200010c0c48656c6c6f2052616469782120220022010121012007408" +
                    "39ac9c47db45950fc0cd453c5ebbbfa7ae5f7c20753abe2370b5b40fdee89e522c4d810d060e0c" +
                    "56211d036043fd32b9908e97bf114c1835ca02d74018fdd09",
            NotarizedTransaction.sample().compile().bytes.hex
        )
    }
}