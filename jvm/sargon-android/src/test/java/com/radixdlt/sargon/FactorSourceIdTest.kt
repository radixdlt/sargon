package com.radixdlt.sargon

import com.radixdlt.sargon.extensions.asGeneral
import com.radixdlt.sargon.extensions.init
import com.radixdlt.sargon.extensions.randomBagOfBytes
import com.radixdlt.sargon.samples.sampleMainnet
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Test

class FactorSourceIdTest {

    @Test
    fun testAsGeneral() {
        val hashFactorSourceId = FactorSourceIdFromHash(
            kind = FactorSourceKind.TRUSTED_CONTACT,
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

}