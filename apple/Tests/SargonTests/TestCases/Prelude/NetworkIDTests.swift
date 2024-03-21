final class NetworkIDTests: Test<NetworkID> {
	func test_non_existing_throws() {
		XCTAssertThrowsError(try SUT(discriminant: 237))
	}
	
	func test_from_raw_value() throws {
		XCTAssertEqual(try SUT(discriminant: 1), SUT.mainnet)
	}
	
	func test_description() {
		XCTAssertNoDifference(SUT.mainnet.description, "mainnet")
	}
}
