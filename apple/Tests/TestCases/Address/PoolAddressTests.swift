final class PoolAddressTests: AddressTest<PoolAddress> {
	func test_pool_kind() {
		XCTAssertNoDifference(SUT.sampleMainnetMulti.poolKind, .multiResources)
		XCTAssertNoDifference(SUT.sampleMainnetTwo.poolKind, .twoResources)
		XCTAssertNoDifference(SUT.sampleMainnetSingle.poolKind, .oneResource)
	}
}
