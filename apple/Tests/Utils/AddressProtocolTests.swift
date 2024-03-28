class BaseAddressTest<SUT_: BaseAddressProtocol>: Test<SUT_> {
	
	func test_network_id_of_sample() {
		XCTAssertNoDifference(SUT.sample.networkID, .mainnet)
	}
	
	func test_network_id_of_sampleOther() {
		XCTAssertNoDifference(SUT.sampleOther.networkID, .mainnet)
	}
	
	func test_all_address_different() {
		XCTAssertEqual(Set(SUT.allCases).count, SUT.allCases.count)
	}
	
	
	func test_bech32_roundtrip() throws {
		func doTest(_ address: SUT) throws {
			try XCTAssertNoDifference(
				SUT(validatingAddress: address.address).address,
				address.address
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

class AddressTest<SUT_: AddressProtocol>: BaseAddressTest<SUT_> {

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
    
    func test_identifiable() {
        SUT.allCases.forEach {
            XCTAssertEqual($0.id, $0.address)
        }
    }
	
	func test_formatted_full_is_address() {
		SUT.allCases.forEach {
			XCTAssertEqual($0.formatted(.full), $0.address)
		}
	}

	func test_xrd_on_same_network_as_address() {
		XCTAssertEqual(SUT.sampleMainnet.xrdOnSameNetwork, ResourceAddress.sampleMainnetXRD)
		XCTAssertEqual(SUT.sampleMainnetOther.xrdOnSameNetwork, ResourceAddress.sampleMainnetXRD)
		XCTAssertEqual(SUT.sampleStokenet.xrdOnSameNetwork, ResourceAddress.sampleStokenetXRD)
		XCTAssertEqual(SUT.sampleStokenetOther.xrdOnSameNetwork, ResourceAddress.sampleStokenetXRD)
	}


	func test_embed() {
		func doTest(_ address: SUT) {
			XCTAssertNoDifference(
				address.embed().address,
				address.address
			)
			
			XCTAssertNoDifference(
				address.embed().networkID,
				address.networkID
			)
		}
		
		SUT.allCases.forEach(doTest)
	}
	
	func test_map_to_same_network_does_not_change() {
		func doTest(_ address: SUT) {
			XCTAssertNoDifference(
				address.mapTo(networkID: address.networkID),
				address
			)
		}
		
		SUT.allCases.forEach(doTest)
	}
	
	
	func test_map_to_other_networks() {
		func doTest(_ address: SUT) {
			NetworkID.allCases.forEach {
				let addressMapped = address.mapTo(networkID: $0)
				XCTAssertEqual(addressMapped.networkID, $0)
				if address.networkID != $0 {
					XCTAssertNotEqual(addressMapped, address)
				}
			}
		}
		
		SUT.allCases.forEach(doTest)
	}
}
