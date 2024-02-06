import uniffi.radix_wallet_kit.*

val placeholder: List<FactorSource>
    get() = newFactorSourcesPlaceholder()

val placeholderOther: List<FactorSource>
    get() = newFactorSourcesPlaceholderOther()

fun test_equals() {
	val a = placeholder
	val b = placeholderOther

	// TODO will change these when HexCoded32Bytes is represented with a list of bytes
	//assert(a == placeholder)
	assert(a != b)
	//assert(b == placeholderOther)
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
