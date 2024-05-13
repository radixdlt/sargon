import CustomDump
import Foundation
import Sargon
import SargonUniFFI
import XCTest

final class LinkConnectionQrDataTests: Test<LinkConnectionQrData> {
    func test_codable() throws {
		let raw = """
		{
			"purpose": "general",
			"password": "deaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddead",
			"publicKey": "ec172b93ad5e563bf4932c70e1245034c35467ef2efd4d64ebf819683467e2bf",
			"signature": "2150c2f6b6c496d197ae03afb23f6adf23b275c675394f23786250abd006d5a2c7543566403cb414f70d0e229b0a9b55b4c74f42fc38cdf1aba2307f97686f0b"
		}
		""".data(using: .utf8)!
		
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