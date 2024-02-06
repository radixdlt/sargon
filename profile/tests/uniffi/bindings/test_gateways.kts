import uniffi.radix_wallet_kit.*

val Gateways.Companion.placeholder
    get() = newGatewaysPlaceholder()

val Gateways.Companion.placeholderOther
    get() = newGatewaysPlaceholderOther()

fun test_equals() {
	val a = Gateways.placeholder
	val b = Gateways.placeholderOther

	assert(a == Gateways.placeholder)
	assert(a != b)
	assert(b == Gateways.placeholderOther)
}

fun test_hashCode() {
	val a = Gateways.placeholder
	val b = Gateways.placeholderOther
	assert(setOf(a, a).size == 1)
	assert(setOf(b, b).size == 1)
	assert(setOf(a, b, b, a).size == 2)
}

fun test_new() {
	val mainnet = gatewayMainnet()
	assert(mainnet == gatewayMainnet())
	val gateways = newGateways(current = mainnet)
	assert(gateways.current.network.id == NetworkId.MAINNET)
}

fun test() {
	test_equals()
	test_hashCode()
	test_new()
}

test()
