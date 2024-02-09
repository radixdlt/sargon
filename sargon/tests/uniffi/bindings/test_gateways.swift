import Sargon

public typealias SUT = Gateways
extension SUT {
	public static let placeholder: Self = newGatewaysPlaceholder()
	public static let placeholderOther: Self = newGatewaysPlaceholderOther()
}

func test_equatable() throws {
	let a = SUT.placeholder
	let b = SUT.placeholderOther
	assert(
		a == SUT.placeholder
	)
	assert(a != b)
	assert(b == SUT.placeholderOther)
}

func test_hashable() throws {
	let a = SUT.placeholder
	let b = SUT.placeholderOther
	assert(Set([a, a]).count == 1)
	assert(Set([b, b]).count == 1)
	assert(Set([a, b, b, a]).count == 2)
}

func test_new() throws {
	let mainnet = gatewayMainnet()
	assert(mainnet == gatewayMainnet())
	let gateways = newGateways(current: mainnet)
	assert(gateways.current.network.id == .mainnet)
}


func test() throws {
	try test_equatable()
	try test_hashable()
	try test_new()
}


try! test()
