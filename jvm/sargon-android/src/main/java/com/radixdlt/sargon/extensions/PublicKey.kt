package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.BagOfBytes
import com.radixdlt.sargon.PublicKey
import com.radixdlt.sargon.ed25519PublicKeyToBytes
import com.radixdlt.sargon.ed25519PublicKeyToHex
import com.radixdlt.sargon.newEd25519PublicKeyFromBytes
import com.radixdlt.sargon.newEd25519PublicKeyFromHex
import com.radixdlt.sargon.newPublicKeyFromBytes
import com.radixdlt.sargon.newPublicKeyFromHex
import com.radixdlt.sargon.newSecp256k1PublicKeyFromBytes
import com.radixdlt.sargon.newSecp256k1PublicKeyFromHex
import com.radixdlt.sargon.publicKeyToBytes
import com.radixdlt.sargon.publicKeyToHex
import com.radixdlt.sargon.secp256k1PublicKeyToBytes
import com.radixdlt.sargon.secp256k1PublicKeyToBytesUncompressed
import com.radixdlt.sargon.secp256k1PublicKeyToHex

@Throws(SargonException::class)
fun PublicKey.Companion.init(hex: String): PublicKey = newPublicKeyFromHex(hex = hex)

@Throws(SargonException::class)
fun PublicKey.Companion.init(bytes: BagOfBytes): PublicKey =
    newPublicKeyFromBytes(bagOfBytes = bytes)

@Throws(SargonException::class)
fun PublicKey.Ed25519.Companion.init(hex: String): PublicKey.Ed25519 =
    PublicKey.Ed25519(newEd25519PublicKeyFromHex(hex = hex))

@Throws(SargonException::class)
fun PublicKey.Secp256k1.Companion.init(hex: String): PublicKey.Secp256k1 =
    PublicKey.Secp256k1(newSecp256k1PublicKeyFromHex(hex = hex))

@Throws(SargonException::class)
fun PublicKey.Ed25519.Companion.init(bytes: BagOfBytes): PublicKey.Ed25519 =
    PublicKey.Ed25519(newEd25519PublicKeyFromBytes(bytes = bytes))

@Throws(SargonException::class)
fun PublicKey.Secp256k1.Companion.init(bytes: BagOfBytes): PublicKey.Secp256k1 =
    PublicKey.Secp256k1(newSecp256k1PublicKeyFromBytes(bytes = bytes))

/**
 * Encodes the [PublicKey.Ed25519] to a hexadecimal string, lowercased, without any `0x` prefix,
 * e.g. `"b7a3c12dc0c8c748ab07525b701122b88bd78f600c76342d27f25e5f92444cde"`
 *
 * Encodes the compressed form (33 bytes) of a [PublicKey.Secp256k1] to a hexadecimal string, 
 * lowercased, without any `0x` prefix, e.g.
 * `"033083620d1596d3f8988ff3270e42970dd2a031e2b9b6488052a4170ff999f3e8"`
 */
val PublicKey.hex: String
    get() = publicKeyToHex(publicKey = this)

val PublicKey.bytes: BagOfBytes
    get() = publicKeyToBytes(publicKey = this)

val PublicKey.Ed25519.bytes: BagOfBytes
    get() = ed25519PublicKeyToBytes(publicKey = value)

/**
 * Returns the key on **compressed** form (33 bytes)
 */
val PublicKey.Secp256k1.bytes: BagOfBytes
    get() = secp256k1PublicKeyToBytes(publicKey = value)


/**
 * Returns the key on **uncompressed** form (65 bytes)
 *
 * Use `compressedData` for compressed format (33 bytes)
 */
val PublicKey.Secp256k1.uncompressedBytes: BagOfBytes
    get() = secp256k1PublicKeyToBytesUncompressed(publicKey = value)
