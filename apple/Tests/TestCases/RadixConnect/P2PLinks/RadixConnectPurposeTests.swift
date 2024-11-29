import CustomDump
import Foundation
import Sargon
import SargonUniFFI
import XCTest

final class RadixConnectPurposeTests: Test<RadixConnectPurpose> {
	func test_string_roundtrip() {
		XCTAssertEqual(
			SUT(rawValue: "general"),
			SUT.general
		)
	}

	func test_codable() throws {
		let raw = "\"general\"".data(using: .utf8)!

		// test decoding
		let sut = try JSONDecoder().decode(SUT.self, from: raw)
		XCTAssertEqual(sut, SUT.sample)

		// test encoding
		let encoded = try JSONEncoder().encode(sut)
		try XCTAssertEqual(JSONDecoder().decode(SUT.self, from: encoded), sut)
	}

	/// Cyon: We might be able remove this function once we have converted to `swift-testing` which has much more
	/// powerful discovery than XCTest, and maybe `eachSampleCodableRoundtripTest` will be picked up as
	/// a test directly.
	func testJSONRoundtripAllSamples() throws {
		try eachSampleCodableRoundtripTest()
	}
}
