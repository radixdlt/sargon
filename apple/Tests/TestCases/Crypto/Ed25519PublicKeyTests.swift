import CryptoKit
import CustomDump
import Foundation
@testable import Sargon
import SargonUniFFI
import XCTest

final class Ed25519PublicKeyTests: PublicKeyTest<Ed25519PublicKey> {
	func test_cryptokit_interop() throws {
		XCTAssertNoThrow(try SUT(hex: Curve25519.Signing.PrivateKey().publicKey.rawRepresentation.hex))
	}

	func test_codable() throws {
		let raw = "\"ec172b93ad5e563bf4932c70e1245034c35467ef2efd4d64ebf819683467e2bf\"".data(using: .utf8)!

		// test decoding
		let sut = try JSONDecoder().decode(SUT.self, from: raw)
		XCTAssertEqual(sut, SUT.sample)

		// test encoding
		let encoded = try JSONEncoder().encode(sut)
		try XCTAssertEqual(JSONDecoder().decode(SUT.self, from: encoded), sut)
	}

	func test_wrapped_in_obj() throws {
		struct Wrapper: Codable, Equatable {
			let myString: String
			let publicKey: Ed25519PublicKey
		}
		let json = """
		{
		    "myString": "Foo",
		    "publicKey": "ec172b93ad5e563bf4932c70e1245034c35467ef2efd4d64ebf819683467e2bf"
		}
		""".data(using: .utf8)!

		let decoded = try JSONDecoder().decode(Wrapper.self, from: json)
		XCTAssertEqual(decoded, try Wrapper(myString: "Foo", publicKey: .init(hex: "ec172b93ad5e563bf4932c70e1245034c35467ef2efd4d64ebf819683467e2bf")))
	}

	/// Cyon: We might be able remove this function once we have converted to `swift-testing` which has much more
	/// powerful discovery than XCTest, and maybe `eachSampleCodableRoundtripTest` will be picked up as
	/// a test directly.
	func testJSONRoundtripAllSamples() throws {
		try eachSampleCodableRoundtripTest()
	}
}
