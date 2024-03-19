class PublicKeyTest<SUT_: PublicKeyProtocol>: Test<SUT_> {
	func test_init_from_hex() throws {
		try XCTAssertEqual(SUT(hex: SUT.sample.hex), SUT.sample)
		try XCTAssertEqual(SUT(hex: SUT.sampleOther.hex), SUT.sampleOther)
	}
}
