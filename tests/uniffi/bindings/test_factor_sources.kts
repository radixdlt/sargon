import com.radixdlt.sargon.*

val placeholder: List<FactorSource>
    get() = newFactorSourcesPlaceholder()

val placeholderOther: List<FactorSource>
    get() = newFactorSourcesPlaceholderOther()

fun test_equals() {
	val a = placeholder
	val b = placeholderOther

	assert(a == a)
	assert(b == b)
	assert(b != a)
	assert(b != a)
}

fun test_hashCode() {
	val a = placeholder
	val b = placeholderOther
	assert(setOf(a, a).size == 1)
	assert(setOf(b, b).size == 1)
	assert(setOf(a, b, b, a).size == 2)
}

fun test() {
	test_equals()
	test_hashCode()
}

test()
