final class LegacyOlympiaAccountAddressTests: BaseAddressTest<LegacyOlympiaAccountAddress> {
	func test_isLegacyOfBabylonAddress() {
		let babylon: AccountAddress = "account_rdx168e8u653alt59xm8ple6khu6cgce9cfx9mlza6wxf7qs3wwdh0pwrf"
		XCTAssert(SUT.sample.isLegacyOfBabylonAddress(babylon))
	}
	func test_to_babylon_address() {
		XCTAssertEqual(SUT.sample.toBabylonAddress(), "account_rdx168e8u653alt59xm8ple6khu6cgce9cfx9mlza6wxf7qs3wwdh0pwrf")
	}
}


