import CustomDump
import Foundation
import Sargon
import SargonUniFFI
import XCTest

final class FiatCurrencyTests: Test<FiatCurrency> {
	func test_low_level_to_json_string() {
		let sut = SUT.sample
		let jsonString = fiatCurrencyToJsonString(fiatCurrency: sut)
		XCTAssertEqual(jsonString, "usd")
	}

	func test_codable() throws {
		let raw = "\"usd\"".data(using: .utf8)!

		// test decoding
		let sut = try JSONDecoder().decode(SUT.self, from: raw)
		XCTAssertEqual(sut, SUT.usd)

		// test encoding
		let encoded = try JSONEncoder().encode(sut)
		try XCTAssertEqual(JSONDecoder().decode(SUT.self, from: encoded), sut)
	}

	func test_wrapped_in_obj() throws {
		struct Wrapper: Codable, Equatable {
			let myString: String
			let sut: SUT
		}
		let json = """
		{
			"myString": "Foo",
			"sut": "usd"
		}
		""".data(using: .utf8)!

		let decoded = try JSONDecoder().decode(Wrapper.self, from: json)
		XCTAssertEqual(decoded, Wrapper(myString: "Foo", sut: .usd))
	}

	/// Cyon: We might be able remove this function once we have converted to `swift-testing` which has much more
	/// powerful discovery than XCTest, and maybe `eachSampleCodableRoundtripTest` will be picked up as
	/// a test directly.
	func testJSONRoundtripAllSamples() throws {
		try eachSampleCodableRoundtripTest()
	}

	func test_raw_value_roundtrip() {
		eachSample { sut in
			XCTAssertEqual(sut.rawValue, sut.jsonStringLiteral())
		}
	}
}
