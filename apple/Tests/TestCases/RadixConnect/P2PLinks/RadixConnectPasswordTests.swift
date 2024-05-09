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
}