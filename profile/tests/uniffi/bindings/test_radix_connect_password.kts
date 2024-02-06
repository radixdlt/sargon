import radix.wallet.kit.*
import kotlin.random.Random

fun randomByteArray(byteCount: Int): ByteArray {
    val bytes = ByteArray(byteCount)
    return Random.nextBytes(bytes)
}

fun test() {
    val byteArray = randomByteArray(byteCount = 32)
    val password = RadixConnectPassword(value = newHex32BytesFrom(bytes = byteArray))

    // TODO will change these when HexCoded32Bytes is represented with a list of bytes
    //assert(password.value == bytes)
    assert(password.value.bytes.contentEquals(byteArray))
}

test()
