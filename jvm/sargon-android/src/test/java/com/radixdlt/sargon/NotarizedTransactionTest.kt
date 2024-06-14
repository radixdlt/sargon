package com.radixdlt.sargon

import com.radixdlt.sargon.extensions.bytes
import com.radixdlt.sargon.extensions.compile
import com.radixdlt.sargon.extensions.hex
import com.radixdlt.sargon.samples.Sample
import com.radixdlt.sargon.samples.sample
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Test

class NotarizedTransactionTest : SampleTestable<NotarizedTransaction> {
    override val samples: List<Sample<NotarizedTransaction>>
        get() = listOf(NotarizedTransaction.sample)

    @Test
    fun testGetData() {
        assertEquals(
            "4d22030221022104210707010a872c0100000000000a912c01000000000009092f24002201012" +
                    "00720ec172b93ad5e563bf4932c70e1245034c35467ef2efd4d64ebf819683467e2bf010108000" +
                    "02022044103800051c9a978fb5bfa066a3e5658251ee3304fb9bf58c35b61f8c10e0e7b91840c0" +
                    "86c6f636b5f6665652101850000fda0c4277708000000000000000000000000000000004103800" +
                    "051c9a978fb5bfa066a3e5658251ee3304fb9bf58c35b61f8c10e0e7b91840c087769746864726" +
                    "177210280005da66318c6318c61f5a61b4c6318c6318cf794aa8d295f14e6318c6318c68500004" +
                    "43945309a7a48000000000000000000000000000000000280005da66318c6318c61f5a61b4c631" +
                    "8c6318cf794aa8d295f14e6318c6318c6850000443945309a7a480000000000000000000000000" +
                    "000004103800051ac224ee242c339b5ea5f1ae567f0520a6ffa24b52a10b8e6cd96a8347f0c147" +
                    "472795f6465706f7369745f6f725f61626f7274210281000000002200002020040720acedaceda" +
                    "cedacedacedacedacedacedacedacedacedacedacedacedacedaced0720babebabebabebabebab" +
                    "ebabebabebabebabebabebabebabebabebabebabebabe0720cafecafecafecafecafecafecafec" +
                    "afecafecafecafecafecafecafecafecafe0720deaddeaddeaddeaddeaddeaddeaddeaddeaddea" +
                    "ddeaddeaddeaddeaddeaddead22010121020c0a746578742f706c61696e2200010c0c48656c6c6" +
                    "f2052616469782120220022010121012007404384757c644c9538630f92feb9a85786c1686af98" +
                    "13bee329f5b0aa976c73b2e7b11c44ac9672948880b7145d176b7b8cdcfdf201f5f0c5f8efa72f" +
                    "5dd1af30d",
            NotarizedTransaction.sample().compile().bytes.hex
        )
    }
}