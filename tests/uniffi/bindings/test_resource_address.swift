import Sargon

func test() throws {
	let bech32 =
		"resource_rdx1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxradxrd"
	let address = try newResourceAddress(bech32: bech32)
	assert(address.address == bech32)
	assert(address.networkId == .mainnet)
}

try! test()
