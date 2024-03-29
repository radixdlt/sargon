final class NonFungibleResourceIndicatorTests: Test<NonFungibleResourceIndicator> {
	func test_ids() {
		XCTAssertFalse(SUT.sample.ids.isEmpty)
	}
}
