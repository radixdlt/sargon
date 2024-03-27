package com.radixdlt.sargon

import com.radixdlt.sargon.extensions.publicKey
import com.radixdlt.sargon.extensions.signature
import com.radixdlt.sargon.samples.Sample
import com.radixdlt.sargon.samples.sample
import org.junit.jupiter.api.Assertions
import org.junit.jupiter.api.Test

class SignatureWithPublicKeyTest: SampleTestable<SignatureWithPublicKey> {
    override val samples: List<Sample<SignatureWithPublicKey>>
        get() = listOf(SignatureWithPublicKey.sample)

    @Test
    fun testSignatureRoundtrip() {
        Assertions.assertEquals(
            SignatureWithPublicKey.sample(),
            with(SignatureWithPublicKey.sample()) {
                SignatureWithPublicKey.Ed25519(
                    publicKey = (publicKey as PublicKey.Ed25519).value,
                    signature = (signature as Signature.Ed25519).value
                )
            }
        )
    }
}