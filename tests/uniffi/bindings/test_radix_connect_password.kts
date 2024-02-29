import com.radixdlt.sargon.*
import kotlin.random.Random

val RadixConnectPassword.Companion.sample
    get() = newRadixConnectPasswordSample()

val RadixConnectPassword.Companion.sampleOther
    get() = newRadixConnectPasswordSampleOther()

fun randomByteArray(byteCount: Int): ByteArray {
    val bytes = ByteArray(byteCount)
    return Random.nextBytes(bytes)
}

fun ByteArray.toBagOfBytes() = newBagOfBytesFrom(bytes = this)


fun test() {
    val byteArray = randomByteArray(byteCount = 32).toBagOfBytes()
    // test identity
    assert(
            RadixConnectPassword(value = newExactly32Bytes(bytes = byteArray)) ==
                    RadixConnectPassword(value = newExactly32Bytes(bytes = byteArray))
    )
    assert(RadixConnectPassword.sample == RadixConnectPassword.sample)
    assert(RadixConnectPassword.sampleOther == RadixConnectPassword.sampleOther)

    // inequality
    assert(RadixConnectPassword.sampleOther != RadixConnectPassword.sample)
    assert(RadixConnectPassword.sample != RadixConnectPassword.sampleOther)
}

test()

