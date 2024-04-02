package com.radixdlt.sargon.samples

import com.radixdlt.sargon.annotation.UsesSampleValues
import com.radixdlt.sargon.SignatureWithPublicKey
import com.radixdlt.sargon.newSignatureWithPublicKeySample
import com.radixdlt.sargon.newSignatureWithPublicKeySampleOther

@UsesSampleValues
val SignatureWithPublicKey.Companion.sample: Sample<SignatureWithPublicKey>
    get() = object : Sample<SignatureWithPublicKey> {

        override fun invoke(): SignatureWithPublicKey = newSignatureWithPublicKeySample()

        override fun other(): SignatureWithPublicKey = newSignatureWithPublicKeySampleOther()
    }