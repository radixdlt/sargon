import CustomDump
import Foundation
import Sargon
import SargonUniFFI
import XCTest

// MARK: - BaseAddressTest
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
		try eachSample { address in
			try XCTAssertNoDifference(
				SUT(validatingAddress: address.address).address,
				address.address
			)
		}
	}

	func test_description_is_bech32() {
		eachSample { address in
			XCTAssertNoDifference(
				address.description,
				address.address
			)
		}
	}
}

// MARK: - AddressTest
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

	/// Cyon: We might be able remove this function once we have converted to `swift-testing` which has much more
	/// powerful discovery than XCTest, and maybe `eachSampleCodableRoundtripTest` will be picked up as
	/// a test directly.
	func testJSONRoundtripAllSamples() throws {
		try eachSampleCodableRoundtripTest()
	}

	func test_identifiable() {
		for sampleValue in SUT.sampleValues {
			XCTAssertEqual(sampleValue.id, sampleValue.address)
		}
	}

	func test_formatted_full_is_address() {
		for sampleValue in SUT.sampleValues {
			XCTAssertEqual(sampleValue.formatted(.full), sampleValue.address)
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
		eachSample { address in
			XCTAssertNoDifference(
				address.asGeneral.address,
				address.address
			)

			XCTAssertNoDifference(
				address.asGeneral.networkID,
				address.networkID
			)
		}
	}

	func test_map_to_same_network_does_not_change() {
		eachSample { address in
			XCTAssertNoDifference(
				address.mapTo(networkID: address.networkID),
				address
			)
		}
	}

	func test_map_to_other_networks() {
		eachSample { address in
			for sampleValue in NetworkID.sampleValues {
				let addressMapped = address.mapTo(networkID: sampleValue)
				XCTAssertEqual(addressMapped.networkID, sampleValue)
				if address.networkID != sampleValue {
					XCTAssertNotEqual(addressMapped, address)
				}
			}
		}
	}

	func test_asymmetric_type_equality() {
		for sampleValue in SUT.sampleValues {
			XCTAssertTrue(sampleValue.asGeneral == sampleValue)
			XCTAssertTrue(sampleValue == sampleValue.asGeneral)
		}
	}

	func test_random() {
		let n = 10
		var set = Set<SUT>()
		let networks = NetworkID.sampleValues
		for networkID in networks {
			for _ in 0 ..< n {
				set.insert(SUT.random(networkID: networkID))
			}
		}
		XCTAssertEqual(set.count, n * networks.count)
	}
}
