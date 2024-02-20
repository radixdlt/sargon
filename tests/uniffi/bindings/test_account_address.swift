import Sargon

extension AccountAddress {
	public var address: String {
		accountAddressBech32Address(address: self)
	}

	public var networkId: NetworkId {
		accountAddressNetworkId(address: self)
	}
}

func testAddress() throws {
	let bech32 = "account_rdx129qdd2yp9vs8jkkn2uwn6sw0ejwmcwr3r4c3usr2hp0nau67m2kzdm"
	let key = try newEd25519PublicKeyFromHex(
		hex: "3e9b96a2a863f1be4658ea66aa0584d2a8847d4c0f658b20e62e3594d994d73d")
	let address0 = newAccountAddressFrom(
		publicKey: PublicKey.ed25519(value: key), networkId: .mainnet)
	assert(address0.address == bech32)
	let address1 = try newAccountAddress(bech32: bech32)
	assert(address1.address == bech32)
	assert(
		accountAddressToShort(
			address: address1
		) == "acco...m2kzdm")
	assert(address1.networkId == .mainnet)
}

func test() throws {
	try testAddress()
}

try! test()
