package com.radixdlt.sargon

import com.radixdlt.sargon.extensions.init
import com.radixdlt.sargon.extensions.publicKey
import com.radixdlt.sargon.extensions.signature
import com.radixdlt.sargon.samples.sample
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Test

class WalletToDappInteractionAuthProofTest {

    @Test
    fun testFromPublicKey() {
        val signatureWithPublicKey = SignatureWithPublicKey.sample()

        assertEquals(
            WalletToDappInteractionAuthProof(
                publicKey = signatureWithPublicKey.publicKey,
                curve = Slip10Curve.CURVE25519,
                signature = signatureWithPublicKey.signature
            ),
            WalletToDappInteractionAuthProof.init(signatureWithPublicKey)
        )

    }

}