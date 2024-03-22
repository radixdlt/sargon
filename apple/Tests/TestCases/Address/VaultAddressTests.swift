final class VaultAddressTests: AddressTest<VaultAddress> {
	func test_is_fungible() {
		XCTAssertTrue(SUT.sampleMainnetFungible.isFungible)
		XCTAssertFalse(SUT.sampleMainnetNonFungible.isFungible)
	}
	
	func test_is_non_fungible() {
		XCTAssertFalse(SUT.sampleMainnetFungible.isNonFungible)
		XCTAssertTrue(SUT.sampleMainnetNonFungible.isNonFungible)
	}
}
