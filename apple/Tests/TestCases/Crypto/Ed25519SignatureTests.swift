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
		let raw = "\"2150c2f6b6c496d197ae03afb23f6adf23b275c675394f23786250abd006d5a2c7543566403cb414f70d0e229b0a9b55b4c74f42fc38cdf1aba2307f97686f0b\"".data(using: .utf8)!

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
            "signature": "2150c2f6b6c496d197ae03afb23f6adf23b275c675394f23786250abd006d5a2c7543566403cb414f70d0e229b0a9b55b4c74f42fc38cdf1aba2307f97686f0b"
        }
        """.data(using: .utf8)!
        
        let decoded = try JSONDecoder().decode(Wrapper.self, from: json)
        XCTAssertEqual(decoded, try Wrapper.init(myString: "Foo", signature: .init(hex: "2150c2f6b6c496d197ae03afb23f6adf23b275c675394f23786250abd006d5a2c7543566403cb414f70d0e229b0a9b55b4c74f42fc38cdf1aba2307f97686f0b")))
    }
}

