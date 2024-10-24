package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.Ed25519Signature
import com.radixdlt.sargon.Exactly32Bytes
import com.radixdlt.sargon.Hash
import com.radixdlt.sargon.NotarySignature
import com.radixdlt.sargon.PublicKey
import com.radixdlt.sargon.SignedTransactionIntentHash
import com.radixdlt.sargon.androidNotarizeHashWithPrivateKeyBytes
import com.radixdlt.sargon.androidSecretKeyGetPublicKeyFromPrivateKeyBytes
import com.radixdlt.sargon.androidSignHashWithPrivateKeyBytes
import com.radixdlt.sargon.annotation.KoverIgnore

class Curve25519SecretKey(
    private val exactly32Bytes: Exactly32Bytes
) {

    companion object {
        fun secureRandom(): Curve25519SecretKey =
            Curve25519SecretKey(exactly32Bytes = Exactly32Bytes.init(randomBagOfBytes(32)))
    }

    @Throws(SargonException::class)
    fun notarize(signedTransactionIntentHash: SignedTransactionIntentHash): NotarySignature =
        androidNotarizeHashWithPrivateKeyBytes(
            privateKeyBytes = exactly32Bytes,
            signedIntentHash = signedTransactionIntentHash
        )

    @Throws(SargonException::class)
    fun sign(hash: Hash): Ed25519Signature =
        androidSignHashWithPrivateKeyBytes(
            privateKeyBytes = exactly32Bytes,
            hash = hash
        )

    @Throws(SargonException::class)
    fun toPublicKey(): PublicKey.Ed25519 =
        androidSecretKeyGetPublicKeyFromPrivateKeyBytes(privateKeyBytes = exactly32Bytes).asGeneral()

    override fun equals(other: Any?): Boolean {
        if (this === other) return true
        if (javaClass != other?.javaClass) return false

        other as Curve25519SecretKey

        return exactly32Bytes == other.exactly32Bytes
    }

    override fun hashCode(): Int {
        return exactly32Bytes.hashCode()
    }

    @KoverIgnore
    override fun toString(): String {
        return "Curve25519SecretKey(exactly32Bytes=$exactly32Bytes)"
    }


}