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

	func test_codable_roundtrip() throws {
		try SUT.sampleValues.forEach(doTestCodableRoundtrip)
	}

	func test_raw_value_roundtrip() {
		func doTest(_ sut: SUT) {
			XCTAssertEqual(sut.rawValue, sut.jsonStringLiteral())
		}
		SUT.sampleValues.forEach(doTest)
	}
}
