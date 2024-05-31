import CustomDump
import Foundation
import Sargon
import SargonUniFFI
import XCTest

final class DeviceInfoTests: Test<DeviceInfo> {
	
	func test_new_iphone() {
		XCTAssertNotEqual(SUT.sample, SUT.iPhone())
	}
	
	func test_not_codable_but_lower_level_json_methods_json_data_roundtrip() throws{
		let sut = SUT.sample
		let json = sut.jsonData()
		try XCTAssertEqual(
			SUT(jsonData: json),
			sut
		)
	}
	
	func test_codable_lowercase_rust_styled_uuid() throws {
        func doTest(_ jsonString: String, expected: SUT? = .sample) throws {
            
            // No matter which encoding strategy is set on encoder/decoder
            // the Date coding should work since it should happen inside of
            // sargon
            func doDoTest(encoder: JSONEncoder, decoder: JSONDecoder) throws {
                let raw = Data(jsonString.utf8)
                // test decoding
                let sut = try decoder.decode(SUT.self, from: raw)
                
                if let expected {
                    XCTAssertEqual(sut, expected)
                }
                
                // test encoding
                let encoded = try encoder.encode(sut)
                try XCTAssertEqual(decoder.decode(SUT.self, from: encoded), sut)
            }
            
            try doDoTest(encoder: .init(), decoder: .init())
            try doDoTest(encoder: .iso8601, decoder: .iso8601)
            try doDoTest(encoder: .init(), decoder: .iso8601)
            try doDoTest(encoder: .iso8601, decoder: .init())
        }
        
        // Rust style:
        // * lower UUID
        // * date with milliseconds
        try doTest("""
        {
            "id": "66f07ca2-a9d9-49e5-8152-77aca3d1dd74",
            "date": "2023-09-11T16:05:56.000Z",
            "description": "iPhone"
        }
        """)

        // Swift style:
        // * uppercase UUID
        // * date without milliseconds
        try doTest("""
        {
            "id": "66F07CA2-A9D9-49E5-8152-77ACA3D1DD74",
            "date": "2023-09-11T16:05:56Z",
            "description": "iPhone"
        }
        """)
        
        // Swift style - new.
        try doTest("""
        {
            "id": "\(UUID().uuidString)",
            "date": "\(Date.now.ISO8601Format())",
            "description": "iPhone"
        }
        """, expected: nil)
	}
	
	func test_date() {
		
		let into_custom: (String) -> Date = { date in
			let df = ISO8601DateFormatter()
			let t: () -> Date? = {
				return df.date(from: date)
			}
			if let d = t() {
				return d
			}
			df.formatOptions.insert(.withFractionalSeconds)
			return t()!
		}
		
		let from_custom: (Date) -> String = { let df = ISO8601DateFormatter();df.formatOptions.insert(.withFractionalSeconds);return df.string(from: $0) }
		
		func testIntoThenFrom(_ vector: (String, String?)) {
			let sut = vector.0
			let expected = vector.1 ?? sut

			let string = from_custom(into_custom(sut))
			
			XCTAssertEqual(string, expected)
		}
		
		func testFromThenInto(_ vector: (String, String?)) {
			let lhs = vector.0
			let rhs = vector.1 ?? lhs
			
			let lhsString = from_custom(into_custom(lhs))
			let rhsString = from_custom(into_custom(rhs))
		
			XCTAssertEqual(rhsString, rhs)
			
			XCTAssertEqual(
				into_custom(lhsString),
				into_custom(rhsString)
			)
		}
		
		let vectors = [
			("2023-12-24T17:13:56.123456Z", "2023-12-24T17:13:56.123Z"), // precision lost (which is OK)
			("2023-12-24T17:13:56.123Z", nil), // unchanged
			("2023-12-24T17:13:56Z", "2023-12-24T17:13:56.000Z") // (000 added, which is OK)
		]
		vectors.forEach(testIntoThenFrom)
		vectors.forEach(testFromThenInto)
	}
    
}

extension JSONEncoder {
    static var iso8601: JSONEncoder {
        let encoder = JSONEncoder()
        encoder.dateEncodingStrategy = .iso8601
        return encoder
    }
}

extension JSONDecoder {
    static var iso8601: JSONDecoder {
        let decoder = JSONDecoder()
        decoder.dateDecodingStrategy = .iso8601
        return decoder
    }
}
