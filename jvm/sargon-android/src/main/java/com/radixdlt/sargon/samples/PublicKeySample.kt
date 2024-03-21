package com.radixdlt.sargon.samples

import androidx.annotation.VisibleForTesting
import androidx.compose.ui.tooling.preview.PreviewParameterProvider
import com.radixdlt.sargon.Ed25519PublicKey
import com.radixdlt.sargon.PublicKey
import com.radixdlt.sargon.Secp256k1PublicKey
import com.radixdlt.sargon.newEd25519PublicKeySample
import com.radixdlt.sargon.newEd25519PublicKeySampleOther
import com.radixdlt.sargon.newPublicKeySample
import com.radixdlt.sargon.newPublicKeySampleOther
import com.radixdlt.sargon.newSecp256k1PublicKeySample
import com.radixdlt.sargon.newSecp256k1PublicKeySampleOther

@VisibleForTesting
val PublicKey.Companion.sample: Sample<PublicKey>
    get() = object : Sample<PublicKey> {

        override fun invoke(): PublicKey = newPublicKeySample()

        override fun other(): PublicKey = newPublicKeySampleOther()
    }

class PublicKeyPreviewParameterProvider: PreviewParameterProvider<PublicKey> {
    override val values: Sequence<PublicKey>
        get() = PublicKey.sample.all.asSequence()

}

@VisibleForTesting
val PublicKey.Ed25519.Companion.sample: Sample<PublicKey.Ed25519>
    get() = object : Sample<PublicKey.Ed25519> {

        override fun invoke(): PublicKey.Ed25519 = PublicKey.Ed25519(newEd25519PublicKeySample())

        override fun other(): PublicKey.Ed25519 = PublicKey.Ed25519(newEd25519PublicKeySampleOther())
    }

class Ed25519PublicKeyPreviewParameterProvider: PreviewParameterProvider<PublicKey.Ed25519> {
    override val values: Sequence<PublicKey.Ed25519>
        get() = PublicKey.Ed25519.sample.all.asSequence()

}

@VisibleForTesting
val PublicKey.Secp256k1.Companion.sample: Sample<PublicKey.Secp256k1>
    get() = object : Sample<PublicKey.Secp256k1> {

        override fun invoke(): PublicKey.Secp256k1 = PublicKey.Secp256k1(newSecp256k1PublicKeySample())

        override fun other(): PublicKey.Secp256k1 = PublicKey.Secp256k1(newSecp256k1PublicKeySampleOther())
    }

class Secp256k1PublicKeyPreviewParameterProvider: PreviewParameterProvider<PublicKey.Secp256k1> {
    override val values: Sequence<PublicKey.Secp256k1>
        get() = PublicKey.Secp256k1.sample.all.asSequence()

}