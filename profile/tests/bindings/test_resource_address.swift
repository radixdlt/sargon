import radix_wallet_kit

func test() throws {
	let bech32 =
		"resource_rdx1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxradxrd"
	let address = try ResourceAddress.fromBech32(string: bech32)
	assert(address.address() == bech32)
	assert(address.networkId() == .mainnet)
}

try! test()
