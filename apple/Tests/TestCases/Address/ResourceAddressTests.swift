final class ResourceAddressTests: AddressTest<ResourceAddress> {
	
	func test_is_fungible() {
		XCTAssertTrue(SUT.sampleMainnetXRD.isFungible)
		XCTAssertFalse(SUT.sampleMainnetNonFungibleGCMembership.isFungible)
	}
	
	func test_is_non_fungible() {
		XCTAssertFalse(SUT.sampleMainnetXRD.isNonFungible)
		XCTAssertTrue(SUT.sampleMainnetNonFungibleGCMembership.isNonFungible)
	}
	
	func test_xrd_on_network() {
		XCTAssertEqual(SUT.xrd(on: .mainnet), SUT.sampleMainnet)
		XCTAssertEqual(SUT.xrd(on: .stokenet), SUT.sampleStokenet)
		XCTAssertEqual(
			SUT.xrd(on: .simulator),
			try! SUT(
				validatingAddress: "resource_sim1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxakj8n3"
			)
		)
		XCTAssertEqual(SUT.sampleMainnetXRD.xrd, SUT.sampleMainnetXRD)
		XCTAssertEqual(SUT.sampleStokenetXRD.xrd, SUT.sampleStokenetXRD)
		
		XCTAssertEqual(AccountAddress.sampleMainnet.xrd, SUT.sampleMainnetXRD)
		XCTAssertEqual(AccountAddress.sampleMainnetOther.xrd, SUT.sampleMainnetXRD)
		XCTAssertEqual(AccountAddress.sampleStokenet.xrd, SUT.sampleStokenetXRD)
		XCTAssertEqual(AccountAddress.sampleStokenetOther.xrd, SUT.sampleStokenetXRD)
		
		XCTAssertEqual(IdentityAddress.sampleMainnet.xrd, SUT.sampleMainnetXRD)
		XCTAssertEqual(IdentityAddress.sampleMainnetOther.xrd, SUT.sampleMainnetXRD)
		XCTAssertEqual(IdentityAddress.sampleStokenet.xrd, SUT.sampleStokenetXRD)
		XCTAssertEqual(IdentityAddress.sampleStokenetOther.xrd, SUT.sampleStokenetXRD)
	}
}
