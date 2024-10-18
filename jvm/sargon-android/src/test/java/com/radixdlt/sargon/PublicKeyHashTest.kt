package com.radixdlt.sargon

import com.radixdlt.sargon.extensions.hex
import com.radixdlt.sargon.extensions.init
import com.radixdlt.sargon.samples.Sample
import com.radixdlt.sargon.samples.sample
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Test

class PublicKeyHashTest: SampleTestable<PublicKeyHash> {

    override val samples: List<Sample<PublicKeyHash>>
        get() = listOf(PublicKeyHash.sample)

    @Test
    fun testInit() {
        val publicKey = PublicKey.init(
            hex = "ec172b93ad5e563bf4932c70e1245034c35467ef2efd4d64ebf819683467e2bf"
        )

        val hash = PublicKeyHash.init(publicKey = publicKey) as PublicKeyHash.Ed25519
        assertEquals(
            "f4e18c034e069baee91ada4764fdfcf2438b8f976861df00557d4cc9e7",
            hash.value.hex
        )
    }
}