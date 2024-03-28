package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.PublicKey
import com.radixdlt.sargon.Signature
import com.radixdlt.sargon.SignatureWithPublicKey
import com.radixdlt.sargon.signatureWithPublicKeyGetPublicKey
import com.radixdlt.sargon.signatureWithPublicKeyGetSignature

val SignatureWithPublicKey.signature: Signature
    get() = signatureWithPublicKeyGetSignature(signatureWithPublicKey = this)

val SignatureWithPublicKey.publicKey: PublicKey
    get() = signatureWithPublicKeyGetPublicKey(signatureWithPublicKey = this)