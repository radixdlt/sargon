import CustomDump
import Foundation
import Sargon
import SargonUniFFI
import XCTest

final class RadixConnectPasswordTests: Test<RadixConnectPassword> {
	func test_codable() throws {
		let raw = "\"deaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddead\"".data(using: .utf8)!

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

	func testMessageHash() throws {
		let hex = "479ae13d3983de8ab520e519cfba01a25fafbbc1e7438ba52e5ed4a40cd2f56a"
		try XCTAssertEqual(
			SUT.sample.messageHash,
			Hash(string: hex)
		)
	}
}
