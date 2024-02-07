import radix.wallet.kit.*

val Profile.Companion.placeholder
    get() = newProfilePlaceholder()

val Profile.Companion.placeholderOther
    get() = newProfilePlaceholderOther()

fun test_equals() {
	val p = Profile.placeholder
	val q = Profile.placeholderOther

	assert(p == Profile.placeholder)
	assert(p == p)
	assert(q == q)
	assert(q == Profile.placeholderOther)
	assert(Profile.placeholder != Profile.placeholderOther)
}

fun test_hashCode() {
	val a = Profile.placeholder
	val b = Profile.placeholderOther

	assert(setOf(a, a).size == 1)
	assert(setOf(b, b).size == 1)
	assert(setOf(a, b, b, a).size == 2)
}

fun test() {
	test_equals()
	test_hashCode()
}


test()
