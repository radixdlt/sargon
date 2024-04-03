package com.radixdlt.sargon.samples

import com.radixdlt.sargon.annotation.UsesSampleValues
import com.radixdlt.sargon.Ed25519Signature
import com.radixdlt.sargon.Secp256k1Signature
import com.radixdlt.sargon.Signature
import com.radixdlt.sargon.newEd25519SignatureSample
import com.radixdlt.sargon.newEd25519SignatureSampleOther
import com.radixdlt.sargon.newSecp256k1SignatureSample
import com.radixdlt.sargon.newSecp256k1SignatureSampleOther
import com.radixdlt.sargon.newSignatureSample
import com.radixdlt.sargon.newSignatureSampleOther

@UsesSampleValues
val Signature.Companion.sample: Sample<Signature>
    get() = object : Sample<Signature> {
        override fun invoke(): Signature = newSignatureSample()

        override fun other(): Signature = newSignatureSampleOther()
    }

@UsesSampleValues
val Ed25519Signature.Companion.sample: Sample<Ed25519Signature>
    get() = object : Sample<Ed25519Signature> {
        override fun invoke(): Ed25519Signature = newEd25519SignatureSample()

        override fun other(): Ed25519Signature = newEd25519SignatureSampleOther()
    }

@UsesSampleValues
val Secp256k1Signature.Companion.sample: Sample<Secp256k1Signature>
    get() = object : Sample<Secp256k1Signature> {
        override fun invoke(): Secp256k1Signature = newSecp256k1SignatureSample()

        override fun other(): Secp256k1Signature = newSecp256k1SignatureSampleOther()
    }