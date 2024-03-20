class AddressTest<SUT_: AddressProtocol>: Test<SUT_> {
	
	func test_network_id_of_mainnet_sample() {
		XCTAssertNoDifference(SUT.sampleMainnet.networkID, .mainnet)
	}
	
	func test_network_id_of_mainnet_sampleOther() {
		XCTAssertNoDifference(SUT.sampleMainnetOther.networkID, .mainnet)
	}
	
	func test_network_id_of_stokenet_sample() {
		XCTAssertNoDifference(SUT.sampleStokenet.networkID, .stokenet)
	}
	
	func test_network_id_of_stokenet_sampleOther() {
		XCTAssertNoDifference(SUT.sampleStokenetOther.networkID, .stokenet)
	}
	
	func test_all_address_different() {
		XCTAssertGreaterThanOrEqual(Set(SUT.allCases).count, 4)
	}

	func test_bech32_roundtrip() throws {
		func doTest(_ address: SUT) throws {
			try XCTAssertNoDifference(
				SUT(validatingAddress: address.address),
				address
			)
		}
		
		try SUT.allCases.forEach(doTest)
	}
	
	func test_description_is_bech32() {
		func doTest(_ address: SUT) {
			XCTAssertNoDifference(
				address.description,
				address.address
			)
		}
		
		SUT.allCases.forEach(doTest)
	}
}
