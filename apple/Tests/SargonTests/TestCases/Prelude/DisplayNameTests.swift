final class DisplayNameTests: Test<DisplayName> {
	func test_too_long_throws() {
		XCTAssertThrowsError(try SUT(validating: "very much too long a name that really does not fit here."))
	}
}
