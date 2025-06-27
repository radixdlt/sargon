package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.WalletToDappInteractionAuthProof
import com.radixdlt.sargon.SignatureWithPublicKey
import com.radixdlt.sargon.newWalletToDappInteractionAuthProofFromSignatureWithPublicKey

fun WalletToDappInteractionAuthProof.Companion.init(
    signatureWithPublicKey: SignatureWithPublicKey
) = newWalletToDappInteractionAuthProofFromSignatureWithPublicKey(
    signatureWithPublicKey = signatureWithPublicKey
)