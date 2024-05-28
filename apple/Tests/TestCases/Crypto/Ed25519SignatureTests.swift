import CustomDump
import Foundation
import Sargon
import SargonUniFFI
import XCTest

final class Ed25519SignatureTests: SignatureTest<Ed25519Signature> {
	func test_from_exactly_64_bytes() {
		XCTAssertEqual(SUT(exactly: SUT.sample.bytes), SUT.sample)
	}

	func test_as_signature() {
		let sut = SUT.sample
		XCTAssertEqual(sut.signature, Signature.ed25519(value: sut))
	}

	func test_codable() throws {
		let raw =
			"\"fc6a4a15516b886b10f26777094cb1abdccb213c9ebdea7a4bceb83b6fcba50fea181b0136ee5659c3dfae5f771e5b6e6f9abbaa3f0435df0be1f732be965103\""
			.data(using: .utf8)!

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
			let signature: Ed25519Signature
		}
		let json = """
			{
			    "myString": "Foo",
			    "signature": "fc6a4a15516b886b10f26777094cb1abdccb213c9ebdea7a4bceb83b6fcba50fea181b0136ee5659c3dfae5f771e5b6e6f9abbaa3f0435df0be1f732be965103"
			}
			""".data(using: .utf8)!

		let decoded = try JSONDecoder().decode(Wrapper.self, from: json)
		XCTAssertEqual(
			decoded,
			try Wrapper.init(
				myString: "Foo",
				signature: .init(
					hex:
						"fc6a4a15516b886b10f26777094cb1abdccb213c9ebdea7a4bceb83b6fcba50fea181b0136ee5659c3dfae5f771e5b6e6f9abbaa3f0435df0be1f732be965103"
				)))
	}

	/// Cyon: We might be able remove this function once we have converted to `swift-testing` which has much more
	/// powerful discovery than XCTest, and maybe `eachSampleCodableRoundtripTest` will be picked up as
	/// a test directly.
	func testJSONRoundtripAllSamples() throws {
		try eachSampleCodableRoundtripTest()
	}
}
