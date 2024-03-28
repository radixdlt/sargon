final class ResourceAddressTests: AddressTest<ResourceAddress> {
	
	func test_is_fungible() {
		XCTAssertTrue(SUT.sampleMainnetXRD.isFungible)
		XCTAssertFalse(SUT.sampleMainnetNonFungibleGCMembership.isFungible)
	}
	
	func test_is_non_fungible() {
		XCTAssertFalse(SUT.sampleMainnetXRD.isNonFungible)
		XCTAssertTrue(SUT.sampleMainnetNonFungibleGCMembership.isNonFungible)
	}
	
	func test_as_non_fungible() {
		XCTAssertNotNil(SUT.sampleMainnetNonFungibleGCMembership.asNonFungibleResourceAddress)
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
		XCTAssertEqual(SUT.sampleMainnetXRD.xrdOnSameNetwork, SUT.sampleMainnetXRD)
		XCTAssertEqual(SUT.sampleStokenetXRD.xrdOnSameNetwork, SUT.sampleStokenetXRD)
		
		XCTAssertEqual(AccountAddress.sampleMainnet.xrdOnSameNetwork, SUT.sampleMainnetXRD)
		XCTAssertEqual(AccountAddress.sampleMainnetOther.xrdOnSameNetwork, SUT.sampleMainnetXRD)
		XCTAssertEqual(AccountAddress.sampleStokenet.xrdOnSameNetwork, SUT.sampleStokenetXRD)
		XCTAssertEqual(AccountAddress.sampleStokenetOther.xrdOnSameNetwork, SUT.sampleStokenetXRD)
		
		XCTAssertEqual(IdentityAddress.sampleMainnet.xrdOnSameNetwork, SUT.sampleMainnetXRD)
		XCTAssertEqual(IdentityAddress.sampleMainnetOther.xrdOnSameNetwork, SUT.sampleMainnetXRD)
		XCTAssertEqual(IdentityAddress.sampleStokenet.xrdOnSameNetwork, SUT.sampleStokenetXRD)
		XCTAssertEqual(IdentityAddress.sampleStokenetOther.xrdOnSameNetwork, SUT.sampleStokenetXRD)
	}
    
    func test_is_xrd() {
        XCTAssertTrue(ResourceAddress.sampleMainnetXRD.isXRD)
        XCTAssertTrue(ResourceAddress.sampleStokenetXRD.isXRD)

        XCTAssertFalse(ResourceAddress.sampleMainnetCandy.isXRD)
        XCTAssertFalse(ResourceAddress.sampleMainnetNonFungibleGCMembership.isXRD)
        XCTAssertFalse(ResourceAddress.sampleStokenetGum.isXRD)
        XCTAssertFalse(ResourceAddress.sampleStokenetGC.isXRD)
    }
}
