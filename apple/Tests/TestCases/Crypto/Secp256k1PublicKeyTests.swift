import CustomDump
import Foundation
@testable import Sargon
import SargonUniFFI
import XCTest

final class Secp256k1PublicKeyTests: PublicKeyTest<Secp256k1PublicKey> {
	func test_from_compressed() throws {
		// from K1: https://github.com/Sajjon/K1/blob/main/Tests/K1Tests/TestCases/Keys/PublicKey/PublicKeyImportTests.swift#L48
		XCTAssertNoThrow(try SUT(hex: "020202020202020202020202020202020202020202020202020202020202020202"))
	}

	func test_from_uncompressed() throws {
		// from K1: https://github.com/Sajjon/K1/blob/main/Tests/K1Tests/TestCases/Keys/PublicKey/PublicKeyImportTests.swift#L48
		XCTAssertNoThrow(try SUT(hex: "040202020202020202020202020202020202020202020202020202020202020202415456f0fc01d66476251cab4525d9db70bfec652b2d8130608675674cde64b2"))
	}

	func test_uncompressed_and_compresses_equals() throws {
		try XCTAssertNoDifference(
			SUT(hex: "040202020202020202020202020202020202020202020202020202020202020202415456f0fc01d66476251cab4525d9db70bfec652b2d8130608675674cde64b2"),
			SUT(hex: "020202020202020202020202020202020202020202020202020202020202020202")
		)
	}

	func test_uncompressed_from_compressed() throws {
		try XCTAssertNoDifference(
			SUT(hex: "020202020202020202020202020202020202020202020202020202020202020202").uncompressedData.hex,
			"040202020202020202020202020202020202020202020202020202020202020202415456f0fc01d66476251cab4525d9db70bfec652b2d8130608675674cde64b2"
		)
	}

	func test_not_on_curve_33_bytes() throws {
		XCTAssertThrowsError(try SUT(hex: "99deadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeef")) { anyError in
			if let sargonError = anyError as? SargonError {
				switch sargonError {
				case .InvalidSecp256k1PublicKeyPointNotOnCurve:
					break // all good
				default:
					XCTFail("Wrong error case: \(sargonError)")
				}
			} else {
				XCTFail("Wrong error type")
			}
		}
	}

	func test_not_on_curve_65_bytes() throws {
		XCTAssertThrowsError(try SUT(hex: "040000000000000000000000000000000000000000000000000000000000000000fffffffffffffffffffffffffffffffffffffffffffffffffffffffefffffc2e")) { anyError in
			if let sargonError = anyError as? SargonError {
				switch sargonError {
				case .InvalidSecp256k1PublicKeyPointNotOnCurve:
					break // all good
				default:
					XCTFail("Wrong error case")
				}
			} else {
				XCTFail("Wrong error type")
			}
		}
	}
}
