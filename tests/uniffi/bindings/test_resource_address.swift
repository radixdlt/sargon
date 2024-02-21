import Sargon

extension ResourceAddress {
	public var address: String {
		resourceAddressBech32Address(address: self)
	}

	public var networkId: NetworkId {
		resourceAddressNetworkId(address: self)
	}
}

func test() throws {
	let bech32 =
		"resource_rdx1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxradxrd"
	let address = try newResourceAddress(bech32: bech32)
	assert(address.address == bech32)
	assert(address.networkId == .mainnet)
}

try! test()
