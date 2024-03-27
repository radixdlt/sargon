final class Secp256k1SignatureTests: SignatureTest<Secp256k1Signature> {
	
	func test_from_exactly_65_bytes() {
		XCTAssertEqual(SUT(exactly: SUT.sample.bytes), SUT.sample)
	}
}
