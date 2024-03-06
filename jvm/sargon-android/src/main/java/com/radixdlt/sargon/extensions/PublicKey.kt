package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.BagOfBytes
import com.radixdlt.sargon.PublicKey
import com.radixdlt.sargon.ed25519PublicKeyToBytes
import com.radixdlt.sargon.ed25519PublicKeyToHex
import com.radixdlt.sargon.newEd25519PublicKeyFromBytes
import com.radixdlt.sargon.newEd25519PublicKeyFromHex
import com.radixdlt.sargon.newSecp256k1PublicKeyFromBytes
import com.radixdlt.sargon.newSecp256k1PublicKeyFromHex
import com.radixdlt.sargon.secp256k1PublicKeyToBytes
import com.radixdlt.sargon.secp256k1PublicKeyToHex

fun PublicKey.Ed25519.Companion.init(hex: String): PublicKey.Ed25519 =
    PublicKey.Ed25519(newEd25519PublicKeyFromHex(hex = hex))

fun PublicKey.Secp256k1.Companion.init(hex: String): PublicKey.Secp256k1 =
    PublicKey.Secp256k1(newSecp256k1PublicKeyFromHex(hex = hex))

fun PublicKey.Ed25519.Companion.init(bytes: BagOfBytes): PublicKey.Ed25519 =
    PublicKey.Ed25519(newEd25519PublicKeyFromBytes(bytes = bytes.byteArray))

fun PublicKey.Secp256k1.Companion.init(bytes: BagOfBytes): PublicKey.Secp256k1 =
    PublicKey.Secp256k1(newSecp256k1PublicKeyFromBytes(bytes = bytes.byteArray))

val PublicKey.hex: String
    get() = when (this) {
        is PublicKey.Ed25519 -> ed25519PublicKeyToHex(publicKey = this.value)
        is PublicKey.Secp256k1 -> secp256k1PublicKeyToHex(publicKey = this.value)
    }

val PublicKey.bagOfBytes: BagOfBytes
    get() = when (this) {
    is PublicKey.Ed25519 -> ed25519PublicKeyToBytes(publicKey = this.value).toBagOfBytes()
    is PublicKey.Secp256k1 -> secp256k1PublicKeyToBytes(publicKey = this.value).toBagOfBytes()
}