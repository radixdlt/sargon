import sargon.*

val AppPreferences.Companion.placeholder
    get() = newAppPreferencesPlaceholder()

val AppPreferences.Companion.placeholderOther
    get() = newAppPreferencesPlaceholderOther()

fun test_equals() {
	val a = AppPreferences.placeholder
	val b = AppPreferences.placeholderOther

	assert(a == a)
	assert(a == AppPreferences.placeholder)
	assert(b == b)
	assert(b == AppPreferences.placeholderOther)
	assert(b != a)
	assert(b != a)
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
