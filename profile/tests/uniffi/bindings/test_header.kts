import uniffi.radix_wallet_kit.*

val Header.Companion.placeholder
    get() = newHeaderPlaceholder()

val Header.Companion.placeholderOther
    get() = newHeaderPlaceholderOther()

fun test_equals() {
	val a = Header.placeholder
	val b = Header.placeholderOther

	assert(a == Header.placeholder)
	assert(a != b)
	assert(b == Header.placeholderOther)
}

fun test_hashCode() {
	val a = Header.placeholder
	val b = Header.placeholderOther

	assert(setOf(a, a).size == 1)
	assert(setOf(b, b).size == 1)
	assert(setOf(a, b, b, a).size == 2)
}

fun test() {
	test_equals()
	test_hashCode()
}

test()
