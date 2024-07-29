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
				"description": "iPhone (iPhone)"
			},
			"lastUsedOnDevice": {
				"id": "66f07ca2-a9d9-49e5-8152-77aca3d1dd74",
				"date": "2023-09-11T16:05:56.000Z",
				"description": "iPhone (iPhone)"
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
	
	/// Cyon: We might be able remove this function once we have converted to `swift-testing` which has much more 
	/// powerful discovery than XCTest, and maybe `eachSampleCodableRoundtripTest` will be picked up as
	/// a test directly.
	func testJSONRoundtripAllSamples() throws {
		try eachSampleCodableRoundtripTest()
	}
	
    func test_header_list_codable() throws {
        let raw = """
        [
            {
                "snapshotVersion": 100,
                "id": "12345678-bbbb-cccc-dddd-abcd12345678",
                "creatingDevice": {
                    "id": "66f07ca2-a9d9-49e5-8152-77aca3d1dd74",
                    "date": "2023-09-11T16:05:56.000Z",
                    "description": "iPhone (iPhone)"
                },
                "lastUsedOnDevice": {
                    "id": "66f07ca2-a9d9-49e5-8152-77aca3d1dd74",
                    "date": "2023-09-11T16:05:56.000Z",
                    "description": "iPhone (iPhone)"
                },
                "lastModified": "2023-09-11T16:05:56.000Z",
                "contentHint": {
                    "numberOfAccountsOnAllNetworksInTotal": 4,
                    "numberOfPersonasOnAllNetworksInTotal": 0,
                    "numberOfNetworks": 2
                }
            },
            {
                "lastUsedOnDevice" : {
                    "date" : "2023-12-24T17:13:56.123Z",
                    "description" : "Android (Android)",
                    "id" : "F07CA662-D9A9-9E45-1582-ACA773D174DD"
                },
                "id" : "87654321-bbbb-cccc-dddd-87654321dcba",
                "contentHint" : {
                    "numberOfNetworks" : 0,
                    "numberOfAccountsOnAllNetworksInTotal" : 0,
                    "numberOfPersonasOnAllNetworksInTotal" : 0
                },
                "creatingDevice" : {
                    "description" : "Android (Android)",
                    "id" : "F07CA662-D9A9-9E45-1582-ACA773D174DD",
                    "date" : "2023-12-24T17:13:56.123Z"
                },
                "snapshotVersion" : 100,
                "lastModified" : "2023-12-24T17:13:56.123Z"
            }
        ]
        """.data(using: .utf8)!
        
        // test decoding
        let headerList = try JSONDecoder().decode([SUT].self, from: raw)
        XCTAssertNoDifference(headerList, [SUT.sample, SUT.sampleOther])
        
        // test encoding
        let encoded = try JSONEncoder().encode(headerList)
        try XCTAssertEqual(JSONDecoder().decode([SUT].self, from: encoded), headerList)
    }
}
