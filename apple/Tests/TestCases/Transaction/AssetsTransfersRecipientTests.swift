final class AssetsTransfersRecipientTests: Test<AssetsTransfersRecipient> {
	func test_id_is_account_address() {
		XCTAssertEqual(SUT.sample.id, SUT.sample.accountAddress)
	}
	func test_description_is_account_address() {
		XCTAssertEqual(SUT.sample.description, SUT.sample.accountAddress.description)
	}
}
