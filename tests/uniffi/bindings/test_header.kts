import com.radixdlt.sargon.*

val Header.Companion.sample
    get() = newHeaderSample()

val Header.Companion.sampleOther
    get() = newHeaderSampleOther()

fun test_equals() {
    val a = Header.sample
    val b = Header.sampleOther

    assert(a == Header.sample)
    assert(a != b)
    assert(b == Header.sampleOther)
}

fun test_hashCode() {
    val a = Header.sample
    val b = Header.sampleOther

    assert(setOf(a, a).size == 1)
    assert(setOf(b, b).size == 1)
    assert(setOf(a, b, b, a).size == 2)
}

fun test() {
    test_equals()
    test_hashCode()
}

test()
