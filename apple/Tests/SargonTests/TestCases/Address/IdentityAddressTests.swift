final class IdentityAddressTests: AddressTest<IdentityAddress> {
	func test_from_public_key() {
		let publicKey: PublicKey = "6c28952be5cdade99c7dd5d003b6b692714b6b74c5fdb5fdc9a8e4ee1d297838"
		let address = SUT(publicKey: publicKey, networkID: .mainnet)
		XCTAssertEqual(address.address, "identity_rdx12tgzjrz9u0xz4l28vf04hz87eguclmfaq4d2p8f8lv7zg9ssnzku8j")
	}
}
