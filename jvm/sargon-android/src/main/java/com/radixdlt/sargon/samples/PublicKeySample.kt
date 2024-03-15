package com.radixdlt.sargon.samples

import androidx.annotation.VisibleForTesting
import androidx.compose.ui.tooling.preview.PreviewParameterProvider
import com.radixdlt.sargon.Ed25519PublicKey
import com.radixdlt.sargon.Secp256k1PublicKey
import com.radixdlt.sargon.newEd25519PublicKeySample
import com.radixdlt.sargon.newEd25519PublicKeySampleOther
import com.radixdlt.sargon.newSecp256k1PublicKeySample
import com.radixdlt.sargon.newSecp256k1PublicKeySampleOther

@VisibleForTesting
val Ed25519PublicKey.Companion.sample: Sample<Ed25519PublicKey>
    get() = object : Sample<Ed25519PublicKey> {

        override fun invoke(): Ed25519PublicKey = newEd25519PublicKeySample()

        override fun other(): Ed25519PublicKey = newEd25519PublicKeySampleOther()
    }

class Ed25519PublicKeyPreviewParameterProvider: PreviewParameterProvider<Ed25519PublicKey> {
    override val values: Sequence<Ed25519PublicKey>
        get() = Ed25519PublicKey.sample.all.asSequence()

}

@VisibleForTesting
val Secp256k1PublicKey.Companion.sample: Sample<Secp256k1PublicKey>
    get() = object : Sample<Secp256k1PublicKey> {

        override fun invoke(): Secp256k1PublicKey = newSecp256k1PublicKeySample()

        override fun other(): Secp256k1PublicKey = newSecp256k1PublicKeySampleOther()
    }

class Secp256k1PublicKeyPreviewParameterProvider: PreviewParameterProvider<Secp256k1PublicKey> {
    override val values: Sequence<Secp256k1PublicKey>
        get() = Secp256k1PublicKey.sample.all.asSequence()

}