package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.Entropy32Bytes
import com.radixdlt.sargon.NotarySignature
import com.radixdlt.sargon.PublicKey
import com.radixdlt.sargon.SignedIntentHash
import com.radixdlt.sargon.androidNotarizeHashWithPrivateKeyBytes
import com.radixdlt.sargon.androidNotaryKeyGetPublicKeyFromPrivateKeyBytes
import com.radixdlt.sargon.annotation.KoverIgnore

class NotaryPrivateKey internal constructor(
    private val entropy32Bytes: Entropy32Bytes
) {

    companion object {
        fun secureRandom(): NotaryPrivateKey =
            NotaryPrivateKey(entropy32Bytes = Entropy32Bytes.random())
    }

    @Throws(SargonException::class)
    fun notarize(signedIntentHash: SignedIntentHash): NotarySignature =
        androidNotarizeHashWithPrivateKeyBytes(
            privateKeyBytes = entropy32Bytes,
            signedIntentHash = signedIntentHash
        )

    @Throws(SargonException::class)
    fun toPublicKey(): PublicKey.Ed25519 =
        androidNotaryKeyGetPublicKeyFromPrivateKeyBytes(privateKeyBytes = entropy32Bytes).asGeneral()

    override fun equals(other: Any?): Boolean {
        if (this === other) return true
        if (javaClass != other?.javaClass) return false

        other as NotaryPrivateKey

        return entropy32Bytes == other.entropy32Bytes
    }

    override fun hashCode(): Int {
        return entropy32Bytes.hashCode()
    }

    @KoverIgnore
    override fun toString(): String {
        return "NotaryPrivateKey(entropy32Bytes=$entropy32Bytes)"
    }


}