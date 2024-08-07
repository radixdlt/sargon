package com.radixdlt.sargon

import com.radixdlt.sargon.extensions.asGeneral
import com.radixdlt.sargon.extensions.formatted
import com.radixdlt.sargon.extensions.init
import com.radixdlt.sargon.extensions.networkId
import com.radixdlt.sargon.extensions.string
import com.radixdlt.sargon.samples.Sample
import com.radixdlt.sargon.samples.sampleMainnet
import com.radixdlt.sargon.samples.sampleStokenet
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Test

class IdentityAddressTest: SampleTestable<IdentityAddress> {

    override val samples: List<Sample<IdentityAddress>>
        get() = listOf(IdentityAddress.sampleMainnet, IdentityAddress.sampleStokenet)

    @Test
    fun test() {
        val addressString = "identity_rdx122yy9pkfdrkam4evxcwh235c4qc52wujkwnt52q7vqxefhnlen489g"
        val identityAddress = IdentityAddress.init(validatingAddress = addressString)

        assertEquals(addressString, identityAddress.string)
        assertEquals(NetworkId.MAINNET, identityAddress.networkId)

        val key = PublicKey.Ed25519.init(
            hex = "3e9b96a2a863f1be4658ea66aa0584d2a8847d4c0f658b20e62e3594d994d73d"
        )
        assertEquals(
            "identity_rdx12fqdd2yp9vs8jkkn2uwn6sw0ejwmcwr3r4c3usr2hp0nau673z8dg0",
            IdentityAddress.init(publicKey = key, networkId = NetworkId.MAINNET).string
        )
    }

    @Test
    fun testFormat() {
        val addressString = "identity_rdx122yy9pkfdrkam4evxcwh235c4qc52wujkwnt52q7vqxefhnlen489g"
        val address = IdentityAddress.init(validatingAddress = addressString)

        assertEquals("iden...en489g", address.formatted())
        assertEquals(
            addressString,
            address.formatted(format = AddressFormat.FULL)
        )
        assertEquals(
            addressString,
            address.formatted(format = AddressFormat.RAW)
        )
    }

    @Test
    fun testAsGeneral() {
        val address = IdentityAddress.sampleMainnet()

        assertEquals(Address.Identity(address), address.asGeneral())
    }
}