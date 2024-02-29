import com.radixdlt.sargon.*
import kotlin.random.Random

fun randomByteArray(byteCount: Int): ByteArray {
    val bytes = ByteArray(byteCount)
    return Random.nextBytes(bytes)
}

fun ByteArray.toBagOfBytes() = newBagOfBytesFrom(bytes = this)

fun Exactly32Bytes.Companion.init(bytes: ByteArray): Exactly32Bytes = newExactly32Bytes(bytes = bytes.toBagOfBytes())

fun test() {
    val bytes = randomByteArray(byteCount = 32)
    assert(Exactly32Bytes.init(bytes = bytes) == Exactly32Bytes.init(bytes = bytes))
}

test()
