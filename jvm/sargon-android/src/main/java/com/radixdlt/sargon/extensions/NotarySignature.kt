package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.NotarySignature
import com.radixdlt.sargon.Signature
import com.radixdlt.sargon.newNotarySignature
import com.radixdlt.sargon.notarySignatureGetSignature

fun NotarySignature.Companion.init(signature: Signature) = newNotarySignature(signature = signature)

val NotarySignature.signature: Signature
    get() = notarySignatureGetSignature(notarySignature = this)