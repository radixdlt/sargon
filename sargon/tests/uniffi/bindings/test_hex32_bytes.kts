import sargon.*
import kotlin.random.Random

fun randomByteArray(byteCount: Int): ByteArray {
    val bytes = ByteArray(byteCount)
    return Random.nextBytes(bytes)
}

fun Hex32Bytes.Companion.init(bytes: ByteArray): Hex32Bytes = newHex32BytesFrom(bytes = bytes)

fun test() {
    val bytes = randomByteArray(byteCount = 32)
    assert(Hex32Bytes.init(bytes = bytes) == Hex32Bytes.init(bytes = bytes))
}

test()
