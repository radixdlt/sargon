import CustomDump
import Foundation
import Sargon
import SargonUniFFI
import XCTest

class BaseAddressTest<SUT_: BaseAddressProtocol>: Test<SUT_> {
	
	func test_network_id_of_sample() {
		XCTAssertNoDifference(SUT.sample.networkID, .mainnet)
	}
	
	func test_network_id_of_sampleOther() {
		XCTAssertNoDifference(SUT.sampleOther.networkID, .mainnet)
	}
	
	func test_all_address_different() {
		XCTAssertEqual(Set(SUT.sampleValues).count, SUT.sampleValues.count)
	}
	
	
	func test_bech32_roundtrip() throws {
		func doTest(_ address: SUT) throws {
			try XCTAssertNoDifference(
				SUT(validatingAddress: address.address).address,
				address.address
			)
		}
		
		try SUT.sampleValues.forEach(doTest)
	}
	
	func test_description_is_bech32() {
		func doTest(_ address: SUT) {
			XCTAssertNoDifference(
				address.description,
				address.address
			)
		}
		
		SUT.sampleValues.forEach(doTest)
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
	
	func test_asSpecific_self() throws {
		func doTestInto(_ sut: SUT) throws {
			let extracted = try sut.asGeneral.asSpecific(type: SUT.self)
			XCTAssertEqual(extracted, sut)
		}
		try SUT.sampleValues.forEach(doTestInto)
	}
	
	func test_codable_roundtrip() throws {
		try SUT.sampleValues.forEach(doTestCodableRoundtrip)
	}
    
    func test_identifiable() {
        SUT.sampleValues.forEach {
            XCTAssertEqual($0.id, $0.address)
        }
    }
	
	func test_formatted_full_is_address() {
		SUT.sampleValues.forEach {
			XCTAssertEqual($0.formatted(.full), $0.address)
		}
	}

	func test_xrd_on_same_network_as_address() {
		XCTAssertEqual(SUT.sampleMainnet.xrdOnSameNetwork, ResourceAddress.sampleMainnetXRD)
		XCTAssertEqual(SUT.sampleMainnetOther.xrdOnSameNetwork, ResourceAddress.sampleMainnetXRD)
		XCTAssertEqual(SUT.sampleStokenet.xrdOnSameNetwork, ResourceAddress.sampleStokenetXRD)
		XCTAssertEqual(SUT.sampleStokenetOther.xrdOnSameNetwork, ResourceAddress.sampleStokenetXRD)
	}
	
	func test_is_on_mainnet() {
		XCTAssertTrue(SUT.sampleMainnet.isOnMainnet)
		XCTAssertTrue(SUT.sampleMainnetOther.isOnMainnet)
		
		XCTAssertFalse(SUT.sampleStokenet.isOnMainnet)
		XCTAssertFalse(SUT.sampleStokenetOther.isOnMainnet)
		
		let nonMainnets = Set(NetworkID.sampleValues).subtracting(Set([NetworkID.mainnet]))
		nonMainnets.map(SUT.random(networkID:)).map(\.isOnMainnet).forEach { XCTAssertFalse($0) }
	}

	func test_asGeneral() {
		func doTest(_ address: SUT) {
			XCTAssertNoDifference(
				address.asGeneral.address,
				address.address
			)
			
			XCTAssertNoDifference(
				address.asGeneral.networkID,
				address.networkID
			)
		}
		
		SUT.sampleValues.forEach(doTest)
	}
	
	func test_map_to_same_network_does_not_change() {
		func doTest(_ address: SUT) {
			XCTAssertNoDifference(
				address.mapTo(networkID: address.networkID),
				address
			)
		}
		
		SUT.sampleValues.forEach(doTest)
	}
	
	
	func test_map_to_other_networks() {
		func doTest(_ address: SUT) {
			NetworkID.sampleValues.forEach {
				let addressMapped = address.mapTo(networkID: $0)
				XCTAssertEqual(addressMapped.networkID, $0)
				if address.networkID != $0 {
					XCTAssertNotEqual(addressMapped, address)
				}
			}
		}
		
		SUT.sampleValues.forEach(doTest)
	}
	
	
	func test_asymmetric_type_equality() {
		SUT.sampleValues.forEach {
			XCTAssertTrue($0.asGeneral == $0)
			XCTAssertTrue($0 == $0.asGeneral)
		}
	}
	
	func test_random() {
		let n = 10
		var set = Set<SUT>()
		let networks = NetworkID.sampleValues
		networks.forEach { networkID in
			(0..<n).forEach { _ in
				set.insert(SUT.random(networkID: networkID))
			}
		}
		XCTAssertEqual(set.count, n * networks.count)
	}
}