final class LegacyOlympiaAccountAddressTests: BaseAddressTest<LegacyOlympiaAccountAddress> {
	
	func test_isLegacyOfBabylonAddress() {
		let babylon: AccountAddress = "account_rdx168e8u653alt59xm8ple6khu6cgce9cfx9mlza6wxf7qs3wwdh0pwrf"
		XCTAssert(SUT.sample.isLegacyOfBabylonAddress(babylon))
		XCTAssert(babylon.wasMigratedFromLegacyOlympia(address: SUT.sample))
	}
	
	func test_to_babylon_address() {
		XCTAssertEqual(SUT.sample.toBabylonAddress(), "account_rdx168e8u653alt59xm8ple6khu6cgce9cfx9mlza6wxf7qs3wwdh0pwrf")
	}
	
	func test_from_public_key() {
		XCTAssertEqual(
			SUT(publicKey: "026f08db98ef1d0231eb15580da9123db8e25aa1747c8c32e5fd2ec47b8db73d5c"), // ExpressibleByStringLiteral
			"rdx1qspx7zxmnrh36q33av24srdfzg7m3cj65968erpjuh7ja3rm3kmn6hq4j9842" as SUT // ExpressibleByStringLiteral
		)
	}
	
	func test_to_string() {
		XCTAssertNoDifference(SUT.sample, "rdx1qspx7zxmnrh36q33av24srdfzg7m3cj65968erpjuh7ja3rm3kmn6hq4j9842")
	}
    
    func test_formatted() {
        XCTAssertNoDifference(SUT.sampleOther.formatted(.default), "rdx...0xqm2ylge")
        XCTAssertNoDifference(SUT.sampleOther.formatted(.raw), "rdx1qsp8n0nx0muaewav2ksx99wwsu9swq5mlndjmn3gm9vl9q2mzmup0xqm2ylge")
    }
}


