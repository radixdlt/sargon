import CustomDump
import Foundation
import Sargon
import SargonUniFFI
import XCTest

final class P2PLinkTests: Test<P2PLink> {
    func test_codable() throws {
		let raw = """
		{
			"connectionPassword": "cafecafecafecafecafecafecafecafecafecafecafecafecafecafecafecafe",
			"connectionPurpose": "general",
			"publicKey": "37842830eca0d08dd684adcb9705b3a473c0c070a322322b53c35e09a1bff298",
			"displayName": "Chrome on Macbook"
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
