final class MessageTests: Test<Message> {
	func test_string_roundtrip() {
		let text = "Hello Rust from Swift"
		XCTAssertEqual(SUT.plaintext(string: text).plaintext, text)
	}
}
