import CustomDump
import Foundation
import Sargon
import SargonUniFFI
import XCTest

final class Exactly29BytesTests: ExactlyNBytesTest<Exactly29Bytes> {}

final class Exactly32BytesTests: ExactlyNBytesTest<Exactly32Bytes> {
	func test_from_array_literal() {
		let sut: SUT = [0xde, 0xad, 0xde, 0xad, 0xde, 0xad, 0xde, 0xad, 0xde, 0xad, 0xde, 0xad, 0xde, 0xad, 0xde, 0xad, 0xde, 0xad, 0xde, 0xad, 0xde, 0xad, 0xde, 0xad, 0xde, 0xad, 0xde, 0xad, 0xde, 0xad, 0xde, 0xad]
		XCTAssertNoDifference(sut, SUT.sample)
	}
	
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
}

final class Exactly33BytesTests: ExactlyNBytesTest<Exactly33Bytes> {}
final class Exactly64BytesTests: ExactlyNBytesTest<Exactly64Bytes> {}
final class Exactly65BytesTests: ExactlyNBytesTest<Exactly65Bytes> {}
