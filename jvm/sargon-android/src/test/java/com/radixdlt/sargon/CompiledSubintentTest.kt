package com.radixdlt.sargon

import com.radixdlt.sargon.extensions.bytes
import com.radixdlt.sargon.extensions.compile
import com.radixdlt.sargon.extensions.decompile
import com.radixdlt.sargon.extensions.hex
import com.radixdlt.sargon.samples.Sample
import com.radixdlt.sargon.samples.sample
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Test

class CompiledSubintentTest: SampleTestable<CompiledSubintent> {
    override val samples: List<Sample<CompiledSubintent>>
        get() = listOf(CompiledSubintent.sample)

    @Test
    fun testDecompile() {
        val expectedSubintent = Subintent.sample()
        assertEquals(
            expectedSubintent,
            expectedSubintent.compile().decompile()
        )
    }

    @Test
    fun testBytes() {
        assertEquals(
            CompiledSubintent.sample().bytes.hex,
            "4d220104210707010a872c0100000000000a912c01000000000009092f2400220101200720ec172b93ad5e563bf4932c70e1245034c35467ef2efd4d64ebf819683467e2bf01010800002022044103800051c9a978fb5bfa066a3e5658251ee3304fb9bf58c35b61f8c10e0e7b91840c086c6f636b5f6665652101850000fda0c4277708000000000000000000000000000000004103800051c9a978fb5bfa066a3e5658251ee3304fb9bf58c35b61f8c10e0e7b91840c087769746864726177210280005da66318c6318c61f5a61b4c6318c6318cf794aa8d295f14e6318c6318c6850000443945309a7a48000000000000000000000000000000000280005da66318c6318c61f5a61b4c6318c6318cf794aa8d295f14e6318c6318c6850000443945309a7a480000000000000000000000000000004103800051ac224ee242c339b5ea5f1ae567f0520a6ffa24b52a10b8e6cd96a8347f0c147472795f6465706f7369745f6f725f61626f72742102810000000022000020200022010121020c0a746578742f706c61696e2200010c0c48656c6c6f20526164697821"
        )
    }
}