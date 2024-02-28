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

fun test() {
    val byteArray = randomByteArray(byteCount = 32)
    // test identity
    assert(
            RadixConnectPassword(value = newHex32BytesFrom(bytes = byteArray)) ==
                    RadixConnectPassword(value = newHex32BytesFrom(bytes = byteArray))
    )
    assert(RadixConnectPassword.sample == RadixConnectPassword.sample)
    assert(RadixConnectPassword.sampleOther == RadixConnectPassword.sampleOther)

    // inequality
    assert(RadixConnectPassword.sampleOther != RadixConnectPassword.sample)
    assert(RadixConnectPassword.sample != RadixConnectPassword.sampleOther)
}

test()