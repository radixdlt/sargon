package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.IntentSignature
import com.radixdlt.sargon.SignatureWithPublicKey
import com.radixdlt.sargon.intentSignatureGetSignatureWithPublicKey
import com.radixdlt.sargon.newIntentSignatureFromSignatureWithPublicKey

fun IntentSignature.Companion.init(signatureWithPublicKey: SignatureWithPublicKey) =
    newIntentSignatureFromSignatureWithPublicKey(signatureWithPublicKey = signatureWithPublicKey)

val IntentSignature.signatureWithPublicKey: SignatureWithPublicKey
    get() = intentSignatureGetSignatureWithPublicKey(intentSignature = this)