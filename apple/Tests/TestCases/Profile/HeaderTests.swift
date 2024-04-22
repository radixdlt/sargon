import CustomDump
import Foundation
import Sargon
import SargonUniFFI
import XCTest

final class HeaderTests: Test<Header> {
	
	func test_not_codable_but_lower_level_json_methods_json_data_roundtrip() throws{
		let sut = SUT.sample
		let json = sut.jsonData()
		try XCTAssertEqual(
			SUT(jsonData: json),
			sut
		)
	}
	
	func test_codable() throws {
		let raw = """
		{
			"snapshotVersion": 100,
			"id": "12345678-bbbb-cccc-dddd-abcd12345678",
			"creatingDevice": {
				"id": "66f07ca2-a9d9-49e5-8152-77aca3d1dd74",
				"date": "2023-09-11T16:05:56.000Z",
				"description": "iPhone"
			},
			"lastUsedOnDevice": {
				"id": "66f07ca2-a9d9-49e5-8152-77aca3d1dd74",
				"date": "2023-09-11T16:05:56.000Z",
				"description": "iPhone"
			},
			"lastModified": "2023-09-11T16:05:56.000Z",
			"contentHint": {
				"numberOfAccountsOnAllNetworksInTotal": 4,
				"numberOfPersonasOnAllNetworksInTotal": 0,
				"numberOfNetworks": 2
			}
		}
		""".data(using: .utf8)!
		
		// test decoding
		let sut = try JSONDecoder().decode(SUT.self, from: raw)
		XCTAssertEqual(sut, SUT.sample)
		
		// test encoding
		let encoded = try JSONEncoder().encode(sut)
		try XCTAssertEqual(JSONDecoder().decode(SUT.self, from: encoded), sut)
	}
	
	func test_codable_roundtrip() throws {
		try SUT.allCases.forEach(doTestCodableRoundtrip)
	}
	
}
