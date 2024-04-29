import CustomDump
import Foundation
import Sargon
import SargonUniFFI
import XCTest

// MARK: - Exactly29BytesTests
final class Exactly29BytesTests: ExactlyNBytesTest<Exactly29Bytes> {}

// MARK: - Exactly32BytesTests
final class Exactly32BytesTests: ExactlyNBytesTest<Exactly32Bytes> {
	func test_from_array_literal() {
		let sut: SUT = [0xDE, 0xAD, 0xDE, 0xAD, 0xDE, 0xAD, 0xDE, 0xAD, 0xDE, 0xAD, 0xDE, 0xAD, 0xDE, 0xAD, 0xDE, 0xAD, 0xDE, 0xAD, 0xDE, 0xAD, 0xDE, 0xAD, 0xDE, 0xAD, 0xDE, 0xAD, 0xDE, 0xAD, 0xDE, 0xAD, 0xDE, 0xAD]
		XCTAssertNoDifference(sut, SUT.sample)
	}

	func test_codable_roundtrip() throws {
		try SUT.sampleValues.forEach(doTestCodableRoundtrip)
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
}

// MARK: - Exactly33BytesTests
final class Exactly33BytesTests: ExactlyNBytesTest<Exactly33Bytes> {}

// MARK: - Exactly64BytesTests
final class Exactly64BytesTests: ExactlyNBytesTest<Exactly64Bytes> {}

// MARK: - Exactly65BytesTests
final class Exactly65BytesTests: ExactlyNBytesTest<Exactly65Bytes> {}
