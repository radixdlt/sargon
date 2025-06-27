package com.radixdlt.sargon

import com.radixdlt.sargon.extensions.asGeneral
import com.radixdlt.sargon.extensions.fromJson
import com.radixdlt.sargon.extensions.hex
import com.radixdlt.sargon.extensions.init
import com.radixdlt.sargon.extensions.string
import com.radixdlt.sargon.extensions.randomBagOfBytes
import com.radixdlt.sargon.extensions.toJson
import com.radixdlt.sargon.samples.sampleMainnet
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Test

class FactorSourceIdTest {

    @Test
    fun testAsGeneral() {
        val hashFactorSourceId = FactorSourceIdFromHash(
            kind = FactorSourceKind.DEVICE,
            body = Exactly32Bytes.init(randomBagOfBytes(32))
        )
        assertEquals(
            FactorSourceId.Hash(hashFactorSourceId),
            hashFactorSourceId.asGeneral()
        )

//        val addressFactorSourceId = FactorSourceIdFromAddress(
//            kind = FactorSourceKind.TRUSTED_CONTACT,
//            body = AccountAddress.sampleMainnet().string
//        )
//        assertEquals(
//            FactorSourceId.Address(addressFactorSourceId),
//            addressFactorSourceId.asGeneral()
//        )
    }

    @Test
    fun testFactorSourceIdJsonRoundTrip() {
        val hashFactorSourceId = FactorSourceIdFromHash(
            kind = FactorSourceKind.DEVICE,
            body = Exactly32Bytes.init(randomBagOfBytes(32))
        )
        val factorSourceIdHash = FactorSourceId.Hash(hashFactorSourceId) as FactorSourceId
        assertEquals(
            hashFactorSourceId.asGeneral(),
            FactorSourceId.fromJson(factorSourceIdHash.toJson())
        )

//        val addressFactorSourceId = FactorSourceIdFromAddress(
//            kind = FactorSourceKind.TRUSTED_CONTACT,
//            body = AccountAddress.sampleMainnet().asGeneral().string
//        )
//        val factorSourceIdAddress = FactorSourceId.Address(addressFactorSourceId) as FactorSourceId
//        assertEquals(
//            addressFactorSourceId.asGeneral(),
//            FactorSourceId.fromJson(factorSourceIdAddress.toJson())
//        )
    }

    @Test
    fun testFactorSourceIdFromHashAndAddressJsonRoundTrip() {
        val sutHash = FactorSourceIdFromHash(
            kind = FactorSourceKind.DEVICE,
            body = Exactly32Bytes.init(randomBagOfBytes(32))
        ).asGeneral()

        assertEquals(
            sutHash,
            FactorSourceId.Hash.fromJson(sutHash.toJson())
        )

//        val sutAddress = FactorSourceIdFromAddress(
//            kind = FactorSourceKind.TRUSTED_CONTACT,
//            body = AccountAddress.sampleMainnet().string
//        ).asGeneral()
//
//        assertEquals(
//            sutAddress,
//            FactorSourceId.Address.fromJson(sutAddress.toJson())
//        )
    }

    @Test
    fun testKnownFactorSourceId() {
        val mnemonic = Mnemonic.init("equip will roof matter pink blind book anxiety banner elbow sun young")
        val factorSourceId = FactorSourceId.Hash.init(
            kind = FactorSourceKind.DEVICE,
            mnemonicWithPassphrase = MnemonicWithPassphrase(
                mnemonic = mnemonic,
                passphrase = "Radix... just imagine!"
            )
        )

        assertEquals(
            "4af22ea955d53263a712d897a797df8388e13b8e7b3f30d7d7da88028b724d60",
            factorSourceId.value.body.hex
        )
        assertEquals(
            FactorSourceKind.DEVICE,
            factorSourceId.value.kind
        )
    }
}