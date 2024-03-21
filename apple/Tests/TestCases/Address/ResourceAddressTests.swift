final class ResourceAddressTests: AddressTest<ResourceAddress> {
	
	func test_is_fungible() {
		XCTAssertTrue(SUT.sampleMainnetXRD.isFungible)
		XCTAssertFalse(SUT.sampleMainnetNonFungibleGCMembership.isFungible)
	}
	
	func test_is_non_fungible() {
		XCTAssertFalse(SUT.sampleMainnetXRD.isNonFungible)
		XCTAssertTrue(SUT.sampleMainnetNonFungibleGCMembership.isNonFungible)
	}
}
