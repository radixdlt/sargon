package com.radixdlt.sargon.samples

import com.radixdlt.sargon.annotation.UsesSampleValues
import com.radixdlt.sargon.PublicKey
import com.radixdlt.sargon.newEd25519PublicKeySample
import com.radixdlt.sargon.newEd25519PublicKeySampleOther
import com.radixdlt.sargon.newPublicKeySample
import com.radixdlt.sargon.newPublicKeySampleOther
import com.radixdlt.sargon.newSecp256k1PublicKeySample
import com.radixdlt.sargon.newSecp256k1PublicKeySampleOther

@UsesSampleValues
val PublicKey.Companion.sample: Sample<PublicKey>
    get() = object : Sample<PublicKey> {

        override fun invoke(): PublicKey = newPublicKeySample()

        override fun other(): PublicKey = newPublicKeySampleOther()
    }

@UsesSampleValues
val PublicKey.Ed25519.Companion.sample: Sample<PublicKey.Ed25519>
    get() = object : Sample<PublicKey.Ed25519> {

        override fun invoke(): PublicKey.Ed25519 = PublicKey.Ed25519(newEd25519PublicKeySample())

        override fun other(): PublicKey.Ed25519 = PublicKey.Ed25519(newEd25519PublicKeySampleOther())
    }

@UsesSampleValues
val PublicKey.Secp256k1.Companion.sample: Sample<PublicKey.Secp256k1>
    get() = object : Sample<PublicKey.Secp256k1> {

        override fun invoke(): PublicKey.Secp256k1 = PublicKey.Secp256k1(newSecp256k1PublicKeySample())

        override fun other(): PublicKey.Secp256k1 = PublicKey.Secp256k1(newSecp256k1PublicKeySampleOther())
    }