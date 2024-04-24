package com.radixdlt.sargon

import com.radixdlt.sargon.extensions.asGeneral
import com.radixdlt.sargon.extensions.fromJson
import com.radixdlt.sargon.extensions.init
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

        val addressFactorSourceId = FactorSourceIdFromAddress(
            kind = FactorSourceKind.TRUSTED_CONTACT,
            body = AccountAddress.sampleMainnet()
        )
        assertEquals(
            FactorSourceId.Address(addressFactorSourceId),
            addressFactorSourceId.asGeneral()
        )
    }

    @Test
    fun testJsonRoundtrip() {
        val sutHash = FactorSourceIdFromHash(
            kind = FactorSourceKind.DEVICE,
            body = Exactly32Bytes.init(randomBagOfBytes(32))
        ).asGeneral()

        assertEquals(
            sutHash,
            FactorSourceId.Hash.fromJson(sutHash.toJson())
        )

        val sutAddress = FactorSourceIdFromAddress(
            kind = FactorSourceKind.TRUSTED_CONTACT,
            body = AccountAddress.sampleMainnet()
        ).asGeneral()

        assertEquals(
            sutAddress,
            FactorSourceId.Address.fromJson(sutAddress.toJson())
        )
    }
}