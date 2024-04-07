package com.radixdlt.sargon.samples

import com.radixdlt.sargon.Signature
import com.radixdlt.sargon.annotation.UsesSampleValues
import com.radixdlt.sargon.extensions.asGeneral
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
val Signature.Ed25519.Companion.sample: Sample<Signature.Ed25519>
    get() = object : Sample<Signature.Ed25519> {
        override fun invoke(): Signature.Ed25519 = newEd25519SignatureSample().asGeneral()

        override fun other(): Signature.Ed25519 = newEd25519SignatureSampleOther().asGeneral()
    }

@UsesSampleValues
val Signature.Secp256k1.Companion.sample: Sample<Signature.Secp256k1>
    get() = object : Sample<Signature.Secp256k1> {
        override fun invoke(): Signature.Secp256k1 = newSecp256k1SignatureSample().asGeneral()

        override fun other(): Signature.Secp256k1 = newSecp256k1SignatureSampleOther().asGeneral()
    }