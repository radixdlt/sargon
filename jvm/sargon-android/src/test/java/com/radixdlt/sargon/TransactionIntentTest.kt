package com.radixdlt.sargon

import com.radixdlt.sargon.extensions.compile
import com.radixdlt.sargon.extensions.hash
import com.radixdlt.sargon.extensions.hex
import com.radixdlt.sargon.samples.Sample
import com.radixdlt.sargon.samples.sample
import org.junit.jupiter.api.Assertions
import org.junit.jupiter.api.Test

class TransactionIntentTest: SampleTestable<TransactionIntent> {
    override val samples: List<Sample<TransactionIntent>>
        get() = listOf(TransactionIntent.sample)

    @Test
    fun testHash() {
        Assertions.assertEquals(
            "txid_rdx1hqnje3wuuatceq9sl9qg492wr6f4ke8mgt0uekvgytd8055me2gqymecm2",
            TransactionIntent.sample().hash().bech32EncodedTxId
        )
    }

    @Test
    fun testCompile() {
        val s = "4d220104210707010a872c0100000000000a912c01000000000009092f2400220101200720ec172b93" +
                "ad5e563bf4932c70e1245034c35467ef2efd4d64ebf819683467e2bf01010800002022044103800051" +
                "c9a978fb5bfa066a3e5658251ee3304fb9bf58c35b61f8c10e0e7b91840c086c6f636b5f6665652101" +
                "850000fda0c4277708000000000000000000000000000000004103800051c9a978fb5bfa066a3e5658" +
                "251ee3304fb9bf58c35b61f8c10e0e7b91840c087769746864726177210280005da66318c6318c61f5" +
                "a61b4c6318c6318cf794aa8d295f14e6318c6318c6850000443945309a7a4800000000000000000000" +
                "0000000000000280005da66318c6318c61f5a61b4c6318c6318cf794aa8d295f14e6318c6318c68500" +
                "00443945309a7a480000000000000000000000000000004103800051ac224ee242c339b5ea5f1ae567" +
                "f0520a6ffa24b52a10b8e6cd96a8347f0c147472795f6465706f7369745f6f725f61626f7274210281" +
                "000000002200002020040720acedacedacedacedacedacedacedacedacedacedacedacedacedacedac" +
                "edaced0720babebabebabebabebabebabebabebabebabebabebabebabebabebabebabebabe0720cafe" +
                "cafecafecafecafecafecafecafecafecafecafecafecafecafecafecafe0720deaddeaddeaddeadde" +
                "addeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddead22010121020c0a746578742f706c61696e22" +
                "00010c0c48656c6c6f20526164697821"
        Assertions.assertEquals(
            s,
            TransactionIntent.sample().compile().hex
        )
    }
}