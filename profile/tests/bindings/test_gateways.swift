import radix_wallet_kit

func test() throws {
	let mainnet = gatewayMainnet()
	let gateways = newGateways(current: mainnet)
	assert(gateways.current.network.id == .mainnet)
}

try! test()
