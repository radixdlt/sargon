import CustomDump
import Foundation
import Sargon
import SargonUniFFI
import XCTest

final class NetworkIDTests: Test<NetworkID> {
	func test_non_existing_throws() {
		XCTAssertThrowsError(try SUT(discriminant: 237))
	}

	func test_from_raw_value() throws {
		XCTAssertEqual(try SUT(discriminant: 1), SUT.mainnet)
	}

	func test_description() {
		XCTAssertNoDifference(SUT.mainnet.description, "mainnet")
	}

	func test_network_id_all_cases_is_12() {
		XCTAssertEqual(SUT.allCases.count, 12)
	}

	func test_codable() throws {
		try SUT.sampleValues.forEach(doTestCodableRoundtrip)
	}
}
