final class Ed25519PublicKeyTests: Test<Ed25519PublicKey> {
    func test_init_from_hex() throws {
		try XCTAssertEqual(SUT(hex: SUT.sample.hex), SUT.sample)
    }
}
