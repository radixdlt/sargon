import radix_wallet_kit

func test() throws {
	let mainnet = gatewayMainnet()
	let gateways = Gateways.withCurrent(current: mainnet)
	assert(gateways.getCurrent().getNetwork().id() == .mainnet)
}

try! test()
