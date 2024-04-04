final class SargonErrorTests: Test<SargonError> {
	
	func test_error_code() {
		XCTAssertEqual(SUT.UnknownNetworkForId(badValue: 99).errorCode, 10049)
	}
	
	func test_error_message() {
		XCTAssertEqual(SUT.UnknownNetworkForId(badValue: 99).errorMessage, "No network found with id: '99'")
	}
	
	func test_description() {
		let sut = SUT.UnknownNetworkForId(badValue: 99)
		XCTAssertEqual(sut.description, sut.errorMessage)
	}
	
	func test_debug_description() {
		let sut = SUT.UnknownNetworkForId(badValue: 99)
		XCTAssertEqual(sut.debugDescription, "10049: No network found with id: '99'")
	}
}
