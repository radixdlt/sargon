final class PublicKeyHashTests: Test<PublicKeyHash> {
	
	func test_hash_public_key_ed25519() {
		XCTAssertNoDifference(SUT.hash(publicKey: .ed25519(value: .sample)), SUT.sample)
	}
	
	func test_hash_public_key_secp256k1() {
		XCTAssertNoDifference(SUT.hash(publicKey: .secp256k1(value: .sample)), SUT.sampleOther)
	}
	
	func test_hashing_init() {
		XCTAssertNoDifference(SUT(hashing: .secp256k1(value: .sample)), SUT.sampleOther)
	}
}
