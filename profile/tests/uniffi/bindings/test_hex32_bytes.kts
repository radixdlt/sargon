import radix.wallet.kit.*
import kotlin.random.Random

fun randomByteArray(byteCount: Int): ByteArray {
    val bytes = ByteArray(byteCount)
    return Random.nextBytes(bytes)
}

fun Hex32Bytes.Companion.init(bytes: ByteArray): Hex32Bytes = newHex32BytesFrom(bytes = bytes)

fun test() {
    val bytes = randomByteArray(byteCount = 32)
    val hex32 = Hex32Bytes.init(bytes = bytes)

    // TODO will change these when HexCoded32Bytes is represented with a list of bytes
    //assert(hex32.bytes == bytes)
    assert(hex32.bytes.contentEquals(bytes))
}

test()
