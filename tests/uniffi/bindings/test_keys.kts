import sargon.*

fun String.hexToByteArray(): ByteArray {
    check(length % 2 == 0) { "Must have an even length" }

    return chunked(2)
        .map { it.toInt(16).toByte() }
        .toByteArray()
}

fun ByteArray.toHex(): String = joinToString(separator = "") { eachByte -> "%02x".format(eachByte) }

fun PublicKey.toHex(): String = when (this) {
    is PublicKey.Ed25519 -> ed25519PublicKeyToHex(publicKey = this.value)
    is PublicKey.Secp256k1 -> secp256k1PublicKeyToHex(publicKey = this.value)
}

fun PublicKey.toBytes(): ByteArray = when (this) {
    is PublicKey.Ed25519 -> ed25519PublicKeyToBytes(publicKey = this.value)
    is PublicKey.Secp256k1 -> secp256k1PublicKeyToBytes(publicKey = this.value)
}

inline fun <reified K: PublicKey> testKey(hex: String): PublicKey {
	val bytes = hex.hexToByteArray()
	assert(bytes.toHex() == hex)

    val pk0 = when (K::class.java) {
        PublicKey.Ed25519::class.java -> PublicKey.Ed25519(newEd25519PublicKeyFromHex(hex = hex))
        PublicKey.Secp256k1::class.java -> PublicKey.Secp256k1(newSecp256k1PublicKeyFromHex(hex = hex))
        else -> error("Not a valid PublicKey class")
    }
	assert(pk0.toHex() == hex)
	// N.B. `equals` wont work since `toBytes` returns ByteArray, we need to translate it into
    // `BagOfBytes` (currently: `List<UByte>`) for it to work, since UByte != Byte
	assert(pk0.toBytes().contentEquals(bytes))

    val pk1 = when (K::class.java) {
        PublicKey.Ed25519::class.java -> PublicKey.Ed25519(newEd25519PublicKeyFromBytes(bytes = bytes))
        PublicKey.Secp256k1::class.java -> PublicKey.Secp256k1(newSecp256k1PublicKeyFromBytes(bytes = bytes))
        else -> error("Not a valid PublicKey class")
    }
	assert(pk1.toHex() == hex)
    assert(pk1.toBytes().contentEquals(bytes))

	return pk0
}

fun testKeysCurve25519() {
	val hex = "ec172b93ad5e563bf4932c70e1245034c35467ef2efd4d64ebf819683467e2bf"
	val key = testKey<PublicKey.Ed25519>(hex = hex)
	when (key) {
        is PublicKey.Ed25519 -> assert(key.toHex() == hex)
        is PublicKey.Secp256k1 -> error("Expected Ed25519 key")
    }
}

fun testKeysSecp256k1() {
	val hex = "02517b88916e7f315bb682f9926b14bc67a0e4246f8a419b986269e1a7e61fffa7"
	val key = testKey<PublicKey.Secp256k1>(hex = hex)
	when (key) {
	    is PublicKey.Secp256k1 -> assert(key.toHex() == hex)
	    is PublicKey.Ed25519 -> error("Expected secp256k1 key")
	}
}

fun testKeys() {
	testKeysCurve25519()
	testKeysSecp256k1()
}

fun test() {
	testKeys()
}

test()
