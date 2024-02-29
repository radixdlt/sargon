import com.radixdlt.sargon.*

val AppPreferences.Companion.sample
    get() = newAppPreferencesSample()

val AppPreferences.Companion.sampleOther
    get() = newAppPreferencesSampleOther()

fun test_equals() {
    val a = AppPreferences.sample
    val b = AppPreferences.sampleOther

    assert(a == a)
    assert(a == AppPreferences.sample)
    assert(b == b)
    assert(b == AppPreferences.sampleOther)
    assert(b != a)
    assert(b != a)
}

fun test_hashCode() {
    val a = AppPreferences.sample
    val b = AppPreferences.sampleOther
    assert(setOf(a, a).size == 1)
    assert(setOf(b, b).size == 1)
    assert(setOf(a, b, b, a).size == 2)
}

fun test() {
    test_equals()
    test_hashCode()
}

test()
