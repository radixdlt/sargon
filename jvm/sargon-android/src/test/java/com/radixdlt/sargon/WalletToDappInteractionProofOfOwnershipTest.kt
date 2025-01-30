package com.radixdlt.sargon

import com.radixdlt.sargon.extensions.init
import com.radixdlt.sargon.extensions.publicKey
import com.radixdlt.sargon.extensions.signature
import com.radixdlt.sargon.extensions.signatureWithPublicKey
import com.radixdlt.sargon.samples.sample
import com.radixdlt.sargon.samples.sampleStokenet
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Assertions.fail
import org.junit.jupiter.api.Test

class WalletToDappInteractionProofOfOwnershipTest {
    @Test
    fun testNewFromIntentSignatures_Ed25519_Account() {
        val owner = AddressOfAccountOrPersona.Account(AccountAddress.sampleStokenet.invoke())
        val intentSignature = IntentSignature.sample() // Ed25519
        val intentSignatureOfOwner = IntentSignatureOfOwner(owner = owner, intentSignature = intentSignature)
        val sut = WalletToDappInteractionProofOfOwnership.init(intentSignatureOfOwner = intentSignatureOfOwner)
        when (sut) {
            is WalletToDappInteractionProofOfOwnership.Account -> {
                assertEquals(sut.v1.proof.curve, Slip10Curve.CURVE25519)
                assertEquals(sut.v1.proof.publicKey, intentSignature.signatureWithPublicKey.publicKey)
                assertEquals(sut.v1.proof.signature, intentSignature.signatureWithPublicKey.signature)
            }
            is WalletToDappInteractionProofOfOwnership.Persona -> fail("Expected account proof")
        }
    }

    @Test
    fun testNewFromIntentSignatures_Ed25519_Persona() {
        val owner = AddressOfAccountOrPersona.Identity(IdentityAddress.sampleStokenet.invoke())
        val intentSignature = IntentSignature.sample() // Ed25519
        val intentSignatureOfOwner = IntentSignatureOfOwner(owner = owner, intentSignature = intentSignature)
        val sut = WalletToDappInteractionProofOfOwnership.init(intentSignatureOfOwner = intentSignatureOfOwner)
        when (sut) {
            is WalletToDappInteractionProofOfOwnership.Account -> fail("Expected Persona proof")
            is WalletToDappInteractionProofOfOwnership.Persona -> {
                assertEquals(sut.v1.proof.curve, Slip10Curve.CURVE25519)
                assertEquals(sut.v1.proof.publicKey, intentSignature.signatureWithPublicKey.publicKey)
                assertEquals(sut.v1.proof.signature, intentSignature.signatureWithPublicKey.signature)
            }
        }
    }

    @Test
    fun testNewFromIntentSignatures_Secp256k1_Account() {
        val owner = AddressOfAccountOrPersona.Account(AccountAddress.sampleStokenet.invoke())
        val intentSignature = IntentSignature.sample.other() // Secp256k1
        val intentSignatureOfOwner = IntentSignatureOfOwner(owner = owner, intentSignature = intentSignature)
        val sut = WalletToDappInteractionProofOfOwnership.init(intentSignatureOfOwner = intentSignatureOfOwner)
        when (sut) {
            is WalletToDappInteractionProofOfOwnership.Account -> {
                assertEquals(sut.v1.proof.curve, Slip10Curve.SECP256K1)
                assertEquals(sut.v1.proof.publicKey, intentSignature.signatureWithPublicKey.publicKey)
                assertEquals(sut.v1.proof.signature, intentSignature.signatureWithPublicKey.signature)
            }
            is WalletToDappInteractionProofOfOwnership.Persona -> fail("Expected account proof")
        }
    }

    @Test
    fun testNewFromIntentSignatures_Secp256k1_Persona() {
        val owner = AddressOfAccountOrPersona.Identity(IdentityAddress.sampleStokenet.invoke())
        val intentSignature = IntentSignature.sample.other() // Secp256k1
        val intentSignatureOfOwner = IntentSignatureOfOwner(owner = owner, intentSignature = intentSignature)
        val sut = WalletToDappInteractionProofOfOwnership.init(intentSignatureOfOwner = intentSignatureOfOwner)
        when (sut) {
            is WalletToDappInteractionProofOfOwnership.Account -> fail("Expected Persona proof")
            is WalletToDappInteractionProofOfOwnership.Persona -> {
                assertEquals(sut.v1.proof.curve, Slip10Curve.SECP256K1)
                assertEquals(sut.v1.proof.publicKey, intentSignature.signatureWithPublicKey.publicKey)
                assertEquals(sut.v1.proof.signature, intentSignature.signatureWithPublicKey.signature)
            }
        }
    }
}