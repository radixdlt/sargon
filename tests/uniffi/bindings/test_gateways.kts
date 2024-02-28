import com.radixdlt.sargon.*

val Gateways.Companion.sample
    get() = newGatewaysSample()

val Gateways.Companion.sampleOther
    get() = newGatewaysSampleOther()

fun test_equals() {
    val a = Gateways.sample
    val b = Gateways.sampleOther

    assert(a == Gateways.sample)
    assert(a != b)
    assert(b == Gateways.sampleOther)
}

fun test_hashCode() {
    val a = Gateways.sample
    val b = Gateways.sampleOther
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
