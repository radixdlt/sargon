import com.radixdlt.sargon.*

val Profile.Companion.sample
    get() = newProfileSample()

val Profile.Companion.sampleOther
    get() = newProfileSampleOther()

fun test_equals() {
    val p = Profile.sample
    val q = Profile.sampleOther

    assert(p == Profile.sample)
    assert(p == p)
    assert(q == q)
    assert(q == Profile.sampleOther)
    assert(Profile.sample != Profile.sampleOther)
}

fun test_hashCode() {
    val a = Profile.sample
    val b = Profile.sampleOther

    assert(setOf(a, a).size == 1)
    assert(setOf(b, b).size == 1)
    assert(setOf(a, b, b, a).size == 2)
}

fun test() {
    test_equals()
    test_hashCode()
}

test()
