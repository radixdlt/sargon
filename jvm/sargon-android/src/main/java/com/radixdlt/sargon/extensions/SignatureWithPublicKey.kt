package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.Hash
import com.radixdlt.sargon.PublicKey
import com.radixdlt.sargon.Signature
import com.radixdlt.sargon.SignatureWithPublicKey
import com.radixdlt.sargon.signatureWithPublicKeyGetPublicKey
import com.radixdlt.sargon.signatureWithPublicKeyGetSignature
import com.radixdlt.sargon.signatureWithPublicKeyIsValid

val SignatureWithPublicKey.signature: Signature
    get() = signatureWithPublicKeyGetSignature(signatureWithPublicKey = this)

val SignatureWithPublicKey.publicKey: PublicKey
    get() = signatureWithPublicKeyGetPublicKey(signatureWithPublicKey = this)

fun SignatureWithPublicKey.isValid(hash: Hash): Boolean = signatureWithPublicKeyIsValid(
    signatureWithPublicKey = this,
    forHash = hash
)