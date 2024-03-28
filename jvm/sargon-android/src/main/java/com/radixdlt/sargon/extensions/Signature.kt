package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.BagOfBytes
import com.radixdlt.sargon.Ed25519Signature
import com.radixdlt.sargon.Exactly64Bytes
import com.radixdlt.sargon.Exactly65Bytes
import com.radixdlt.sargon.Secp256k1Signature
import com.radixdlt.sargon.Signature
import com.radixdlt.sargon.ed25519SignatureToString
import com.radixdlt.sargon.newEd25519PublicKeyFromBytes
import com.radixdlt.sargon.newEd25519SignatureFromBytes
import com.radixdlt.sargon.newEd25519SignatureFromExactly64Bytes
import com.radixdlt.sargon.newSecp256k1SignatureFromBytes
import com.radixdlt.sargon.newSecp256k1SignatureFromExactly65Bytes
import com.radixdlt.sargon.newSignatureFromBytes
import com.radixdlt.sargon.secp256k1SignatureToString
import com.radixdlt.sargon.signatureToBytes
import com.radixdlt.sargon.signatureToString

@Throws(SargonException::class)
fun Signature.Companion.init(bytes: BagOfBytes) = newSignatureFromBytes(bytes = bytes)

val Signature.string: String
    get() = signatureToString(signature = this)

val Signature.bytes: BagOfBytes
    get() = signatureToBytes(signature = this)

@Throws(SargonException::class)
fun Signature.Ed25519.Companion.init(bytes: BagOfBytes) = Signature.Ed25519(
    value = newEd25519SignatureFromBytes(bytes = bytes)
)

fun Signature.Ed25519.Companion.init(exactly64Bytes: Exactly64Bytes) = Signature.Ed25519(
    value = newEd25519SignatureFromExactly64Bytes(bytes = exactly64Bytes)
)

@Throws(SargonException::class)
fun Signature.Secp256k1.Companion.init(bytes: BagOfBytes) = Signature.Secp256k1(
    value = newSecp256k1SignatureFromBytes(bytes = bytes)
)

fun Signature.Secp256k1.Companion.init(exactly65Bytes: Exactly65Bytes) = Signature.Secp256k1(
    value = newSecp256k1SignatureFromExactly65Bytes(bytes = exactly65Bytes)
)

@Throws(SargonException::class)
fun Ed25519Signature.Companion.init(bytes: BagOfBytes) =
    newEd25519SignatureFromBytes(bytes = bytes)

fun Ed25519Signature.Companion.init(exactly64Bytes: Exactly64Bytes) =
    newEd25519SignatureFromExactly64Bytes(bytes = exactly64Bytes)

val Ed25519Signature.string: String
    get() = ed25519SignatureToString(signature = this)

@Throws(SargonException::class)
fun Secp256k1Signature.Companion.init(bytes: BagOfBytes) =
    newSecp256k1SignatureFromBytes(bytes = bytes)

fun Secp256k1Signature.Companion.init(exactly65Bytes: Exactly65Bytes) =
    newSecp256k1SignatureFromExactly65Bytes(bytes = exactly65Bytes)

val Secp256k1Signature.string: String
    get() = secp256k1SignatureToString(signature = this)


