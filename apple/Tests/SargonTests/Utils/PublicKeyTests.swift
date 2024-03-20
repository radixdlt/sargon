class PublicKeyTest<SUT_: PublicKeyProtocol>: Test<SUT_> {
	func test_init_from_hex() throws {
		try XCTAssertNoDifference(SUT(hex: SUT.sample.hex), SUT.sample)
		try XCTAssertNoDifference(SUT(hex: SUT.sampleOther.hex), SUT.sampleOther)
	}
}
