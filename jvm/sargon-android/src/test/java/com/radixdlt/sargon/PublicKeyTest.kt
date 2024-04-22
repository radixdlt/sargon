package com.radixdlt.sargon

import com.radixdlt.sargon.extensions.asGeneral
import com.radixdlt.sargon.extensions.bytes
import com.radixdlt.sargon.extensions.hex
import com.radixdlt.sargon.extensions.hexToBagOfBytes
import com.radixdlt.sargon.extensions.init
import com.radixdlt.sargon.extensions.isValidSignature
import com.radixdlt.sargon.extensions.uncompressedBytes
import com.radixdlt.sargon.samples.Sample
import com.radixdlt.sargon.samples.sample
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Assertions.assertFalse
import org.junit.jupiter.api.Test

class PublicKeyTest: SampleTestable<PublicKey> {

    override val samples: List<Sample<PublicKey>>
        get() = listOf(PublicKey.sample)

    inline fun <reified K : PublicKey> testKey(hex: String): PublicKey {
        val bytes = hex.hexToBagOfBytes()
        assert(bytes.hex == hex)

        val pk0 = when (K::class.java) {
            PublicKey.Ed25519::class.java -> PublicKey.Ed25519.init(hex = hex)
            PublicKey.Secp256k1::class.java -> PublicKey.Secp256k1.init(hex = hex)
            else -> error("Not a valid PublicKey class")
        }
        assertEquals(hex, pk0.hex)
        // N.B. `equals` wont work since `toBytes` returns ByteArray, we need to translate it into
        // `BagOfBytes` (currently: `List<UByte>`) for it to work, since UByte != Byte
        assertEquals(bytes, pk0.bytes)

        val pk1 = when (K::class.java) {
            PublicKey.Ed25519::class.java -> PublicKey.Ed25519.init(bytes = bytes)
            PublicKey.Secp256k1::class.java -> PublicKey.Secp256k1.init(bytes = bytes)
            else -> error("Not a valid PublicKey class")
        }
        assertEquals(hex, pk1.hex)
        assertEquals(bytes, pk1.bytes)

        return pk0
    }

    @Test
    fun testKeysCurve25519() {
        val hex = "ec172b93ad5e563bf4932c70e1245034c35467ef2efd4d64ebf819683467e2bf"
        when (val key = testKey<PublicKey.Ed25519>(hex = hex)) {
            is PublicKey.Ed25519 -> {
                assertEquals(hex, key.hex)

                assertEquals(key, PublicKey.init(bytes = key.bytes))
            }
            is PublicKey.Secp256k1 -> error("Expected Ed25519 key")
        }
    }

    @Test
    fun testKeysSecp256k1() {
        val hex = "02517b88916e7f315bb682f9926b14bc67a0e4246f8a419b986269e1a7e61fffa7"
        when (val key = testKey<PublicKey.Secp256k1>(hex = hex)) {
            is PublicKey.Secp256k1 -> {
                assertEquals(hex, key.hex)
                assertEquals(
                    "04517b88916e7f315bb682f9926b14bc67a0e4246f8a419b986269e1a7e61fffa71159e5614fb40739f4d22004380670cbc99ee4a2a73899d084098f3a139130c4",
                    key.uncompressedBytes.hex
                )
                assertEquals(key, PublicKey.init(bytes = key.bytes))
            }
            is PublicKey.Ed25519 -> error("Expected secp256k1 key")
        }
    }

    @Test
    fun testAsGeneral() {
        val publicKeyEd25519 = PublicKey.Ed25519.sample()
        assertEquals(publicKeyEd25519, publicKeyEd25519.v1.asGeneral())

        val publicKeySecp256k1 = PublicKey.Secp256k1.sample()
        assertEquals(publicKeySecp256k1, publicKeySecp256k1.v1.asGeneral())
    }

    @Test
    fun testIsValid() {
        assertFalse(PublicKey.sample().isValidSignature(
            signature = Signature.sample(),
            hashedMessage = Hash.sample()
        ))
    }
}