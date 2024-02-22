import com.radixdlt.sargon.*
import kotlin.random.Random


val RadixConnectPassword.Companion.placeholder
    get() = newRadixConnectPasswordPlaceholder()

val RadixConnectPassword.Companion.placeholderOther
    get() = newRadixConnectPasswordPlaceholderOther()


fun randomByteArray(byteCount: Int): ByteArray {
    val bytes = ByteArray(byteCount)
    return Random.nextBytes(bytes)
}

fun test() {
    val byteArray = randomByteArray(byteCount = 32)
    // test identity
    assert(RadixConnectPassword(value = newHex32BytesFrom(bytes = byteArray)) == RadixConnectPassword(value = newHex32BytesFrom(bytes = byteArray)))
    assert(RadixConnectPassword.placeholder == RadixConnectPassword.placeholder)
    assert(RadixConnectPassword.placeholderOther == RadixConnectPassword.placeholderOther)

    // inequality
    assert(RadixConnectPassword.placeholderOther != RadixConnectPassword.placeholder)
    assert(RadixConnectPassword.placeholder != RadixConnectPassword.placeholderOther)
}

test()
