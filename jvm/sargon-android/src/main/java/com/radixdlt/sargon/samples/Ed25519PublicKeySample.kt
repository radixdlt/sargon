package com.radixdlt.sargon.samples

import com.radixdlt.sargon.Ed25519PublicKey
import com.radixdlt.sargon.annotation.UsesSampleValues
import com.radixdlt.sargon.newEd25519PublicKeySample
import com.radixdlt.sargon.newEd25519PublicKeySampleOther

@UsesSampleValues
val Ed25519PublicKey.Companion.sample: Sample<Ed25519PublicKey>
    get() = object : Sample<Ed25519PublicKey> {

        override fun invoke(): Ed25519PublicKey = newEd25519PublicKeySample()

        override fun other(): Ed25519PublicKey = newEd25519PublicKeySampleOther()

    }