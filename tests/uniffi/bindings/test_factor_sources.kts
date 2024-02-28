import com.radixdlt.sargon.*

val sample: List<FactorSource>
    get() = newFactorSourcesSample()

val SampleOther: List<FactorSource>
    get() = newFactorSourcesSampleOther()

fun test_equals() {
    val a = sample
    val b = SampleOther

    assert(a == a)
    assert(b == b)
    assert(b != a)
    assert(b != a)
}

fun test_hashCode() {
    val a = sample
    val b = SampleOther
    assert(setOf(a, a).size == 1)
    assert(setOf(b, b).size == 1)
    assert(setOf(a, b, b, a).size == 2)
}

fun test() {
    test_equals()
    test_hashCode()
}

test()
