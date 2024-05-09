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
			"publicKey": "b7a3c12dc0c8c748ab07525b701122b88bd78f600c76342d27f25e5f92444cde",
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
}
