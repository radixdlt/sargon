package com.radixdlt.sargon

import com.radixdlt.sargon.extensions.init
import com.radixdlt.sargon.extensions.publicKey
import com.radixdlt.sargon.extensions.signature
import com.radixdlt.sargon.extensions.signatureWithPublicKey
import com.radixdlt.sargon.samples.sample
import com.radixdlt.sargon.samples.sampleStokenet
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Test

class WalletToDappInteractionAuthProofTest {

    @Test
    fun testNewFromIntentSignatures_Ed25519() {
        val intentSignature = IntentSignature.sample.invoke() // Ed25519
        val intentSignatureOfOwner = IntentSignatureOfOwner(owner = AddressOfAccountOrPersona.sampleStokenet.invoke(), intentSignature = intentSignature )
        val sut = WalletToDappInteractionAuthProof.init(intentSignatureOfOwner = intentSignatureOfOwner)
        assertEquals(sut.curve, Slip10Curve.CURVE25519)
        assertEquals(sut.publicKey, intentSignature.signatureWithPublicKey.publicKey)
        assertEquals(sut.signature, intentSignature.signatureWithPublicKey.signature)
    }

    @Test
    fun testNewFromIntentSignatures_Secp256k1() {
        val intentSignature = IntentSignature.sample.other() // Secp256k1
        val intentSignatureOfOwner = IntentSignatureOfOwner(owner = AddressOfAccountOrPersona.sampleStokenet.invoke(), intentSignature = intentSignature )
        val sut = WalletToDappInteractionAuthProof.init(intentSignatureOfOwner = intentSignatureOfOwner)
        assertEquals(sut.curve, Slip10Curve.SECP256K1)
        assertEquals(sut.publicKey, intentSignature.signatureWithPublicKey.publicKey)
        assertEquals(sut.signature, intentSignature.signatureWithPublicKey.signature)
    }
}