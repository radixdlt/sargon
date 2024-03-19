import Foundation
import Sargon
import XCTest

class AddressTest<SUT_: AddressProtocol>: Test<SUT_> {
	
	func test_network_id_of_mainnet_sample() {
		XCTAssertEqual(SUT.sampleMainnet.networkID, .mainnet)
	}
	
	func test_network_id_of_mainnet_sampleOther() {
		XCTAssertEqual(SUT.sampleMainnetOther.networkID, .mainnet)
	}
	
	func test_network_id_of_stokenet_sample() {
		XCTAssertEqual(SUT.sampleStokenet.networkID, .stokenet)
	}
	
	func test_network_id_of_stokenet_sampleOther() {
		XCTAssertEqual(SUT.sampleStokenetOther.networkID, .stokenet)
	}
	
	func test_all_address_different() {
		XCTAssertGreaterThanOrEqual(Set(SUT.allCases).count, 4)
	}

	func test_bech32_roundtrip() throws {
		func doTest(_ address: SUT) throws {
			try XCTAssertEqual(
				SUT(validatingAddress: address.address),
				address
			)
		}
		
		try SUT.allCases.forEach(doTest)
	}
	
	func test_description_is_bech32() {
		func doTest(_ address: SUT) {
			XCTAssertEqual(
				address.description,
				address.address
			)
		}
		
		SUT.allCases.forEach(doTest)
	}
}
