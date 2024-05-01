package com.radixdlt.sargon

import com.radixdlt.sargon.extensions.hex
import com.radixdlt.sargon.extensions.notarize
import com.radixdlt.sargon.extensions.signature
import com.radixdlt.sargon.extensions.string
import com.radixdlt.sargon.extensions.toPublicKey
import com.radixdlt.sargon.samples.sample
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Test

class Entropy32BytesTest {

    @Test
    fun testNotarize() {
        val sut = Entropy32Bytes.sample()
        val result = sut.notarize(SignedIntentHash.sample())

        assertEquals(
            "08c6129fa6938a31e38dfe94effdce8f1a4021e22cf62344830d83dc45f32de0e3d112794c369450e62d245a17b18835f40c639033fbb4b1f975ad0ad71dbf0a",
            result.signature.string
        )
    }

    @Test
    fun testGetPublicKey() {
        val sut = Entropy32Bytes.sample()

        assertEquals(
            "248acbdbaf9e050196de704bea2d68770e519150d103b587dae2d9cad53dd930",
            sut.toPublicKey().hex
        )
    }

}