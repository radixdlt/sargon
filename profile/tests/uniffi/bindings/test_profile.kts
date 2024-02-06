import radix.wallet.kit.*

val Profile.Companion.placeholder
    get() = newProfilePlaceholder()

val Profile.Companion.placeholderOther
    get() = newProfilePlaceholderOther()

fun test_equals() {
	val p = Profile.placeholder
	val q = Profile.placeholderOther

    // TODO will change these when HexCoded32Bytes is represented with a list of bytes
	//assert(p == Profile.placeholder)
	assert(p != q)
	// TODO will change these when HexCoded32Bytes is represented with a list of bytes
	//assert(q == Profile.placeholderOther)
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
