import uniffi.radix_wallet_kit.*

val AppPreferences.Companion.placeholder
    get() = newAppPreferencesPlaceholder()

val AppPreferences.Companion.placeholderOther
    get() = newAppPreferencesPlaceholderOther()

fun test_equals() {
	val a = AppPreferences.placeholder
	val b = AppPreferences.placeholderOther

    // TODO will change these when HexCoded32Bytes is represented with a list of bytes
	//assert(a == AppPreferences.placeholder)
	assert(a != b)
	//assert(b == AppPreferences.placeholderOther)
}

fun test_hashCode() {
	val a = AppPreferences.placeholder
	val b = AppPreferences.placeholderOther
	assert(setOf(a, a).size == 1)
	assert(setOf(b, b).size == 1)
	assert(setOf(a, b, b, a).size == 2)
}

fun test() {
	test_equals()
	test_hashCode()
}

test()
