import CustomDump
import Foundation
import Sargon
import SargonUniFFI
import XCTest

final class DeviceInfoTests: Test<DeviceInfo> {
	
	func test_new_iphone() {
		XCTAssertNotEqual(SUT.sample, SUT.iPhone())
	}
	
	func test_not_codable_but_lower_level_json_methods_json_string_roundtrip() throws{
		let sut = SUT.sample
		let json = sut.jsonString()
		try XCTAssertEqual(
			SUT(jsonString: json),
			sut
		)
	}
	
	func test_codable() throws {
		let raw = """
		{
			"id": "66f07ca2-a9d9-49e5-8152-77aca3d1dd74",
			"date": "2023-09-11T16:05:56.000Z",
			"description": "iPhone"
		}
		""".data(using: .utf8)!
		
		let sut = try JSONDecoder().decode(SUT.self, from: raw)
		XCTAssertEqual(sut, SUT.sample)
		let encoded = try JSONEncoder().encode(sut)
		print(String(data: encoded, encoding: .utf8)!)
		try XCTAssertEqual(JSONDecoder().decode(SUT.self, from: encoded), sut)
	}
}
