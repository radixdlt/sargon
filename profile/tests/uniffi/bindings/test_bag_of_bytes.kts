@file:OptIn(kotlin.ExperimentalStdlibApi::class)

import radix.wallet.kit.*
import kotlin.random.Random

fun String.hexToBagOfBytes(): BagOfBytes {
    check(length % 2 == 0) { "Must have an even length" }

    return chunked(2)
        .map { it.toInt(16).toByte() }
        .toByteArray()
        .toBagOfBytes()
}

fun randomBagOfBytes(byteCount: Int): BagOfBytes {
    val bytes = ByteArray(byteCount)
    return Random.nextBytes(bytes).toBagOfBytes()
}

fun ByteArray.toBagOfBytes() = newBagOfBytesFrom(bytes = this)

val BagOfBytes.hex: String
    get() = toUByteArray().toHexString()

val acedBugOfBytes: BagOfBytes
    get() = newBagOfBytesPlaceholderAced()

val babeBugOfBytes: BagOfBytes
    get() = newBagOfBytesPlaceholderBabe()

val cafeBugOfBytes: BagOfBytes
    get() = newBagOfBytesPlaceholderCafe()

val deadBugOfBytes: BagOfBytes
    get() = newBagOfBytesPlaceholderDead()

val ecadBugOfBytes: BagOfBytes
    get() = newBagOfBytesPlaceholderEcad()

val fadeBugOfBytes: BagOfBytes
    get() = newBagOfBytesPlaceholderFade()

fun BagOfBytes.appendingCafe() = bagOfBytesAppendCafe(to = this)
fun BagOfBytes.appendingDeadbeef() = bagOfBytesAppendDeadbeef(to = this)
fun BagOfBytes.prependingCafe() = bagOfBytesPrependCafe(inFrontOf = this)
fun BagOfBytes.prependingDeadbeef() = bagOfBytesPrependDeadbeef(inFrontOf = this)

fun test() {
	var a = ubyteArrayOf().toList()
	var b = ubyteArrayOf().toList()
    assert(a == b)

    a = ubyteArrayOf(129.toUByte()).toList()
    b = byteArrayOf(129.toByte()).toBagOfBytes()
    assert(a == b)

    assert("acedacedacedacedacedacedacedacedacedacedacedacedacedacedacedaced".hexToBagOfBytes() == acedBugOfBytes)
    assert("babebabebabebabebabebabebabebabebabebabebabebabebabebabebabebabe".hexToBagOfBytes() == babeBugOfBytes)
    assert("cafecafecafecafecafecafecafecafecafecafecafecafecafecafecafecafe".hexToBagOfBytes() == cafeBugOfBytes)
    assert("deaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddead".hexToBagOfBytes() == deadBugOfBytes)
    assert("ecadecadecadecadecadecadecadecadecadecadecadecadecadecadecadecad".hexToBagOfBytes() == ecadBugOfBytes)
    assert("fadefadefadefadefadefadefadefadefadefadefadefadefadefadefadefade".hexToBagOfBytes() == fadeBugOfBytes)

    a = "beef".hexToBagOfBytes()
    assert(a.appendingCafe().hex == "beefcafe")
    assert(a.appendingDeadbeef().hex == "beefdeadbeef")
    assert(a.prependingCafe().hex == "cafebeef")
    assert(a.prependingDeadbeef().hex == "deadbeefbeef")

    b = "42".hexToBagOfBytes()
    assert(
        b.appendingCafe().appendingDeadbeef().prependingCafe().prependingDeadbeef().hex == "deadbeefcafe42cafedeadbeef"
    )

    (0.toUByte()..UByte.MAX_VALUE).forEach {
        val bytes = ubyteArrayOf(it.toUByte())
        assert(bytes.toByteArray().toBagOfBytes() == bytes.toList())
    }

    val size = 100
    val set = (0..<size).map {
        randomBagOfBytes(byteCount = 16 + it)
    }.toSet()
    assert(set.size == size)
}

test()
