import CustomDump
import Foundation
import Sargon
import SargonUniFFI
import XCTest

final class PersonaDataEntryNameTests: PersonaDataEntryTest<PersonaDataEntryName> {
	func test_kind() {
		XCTAssertEqual(SUT.kind, .fullName)
		func doTest(_ sut: SUT) {
			XCTAssertEqual(sut.kind, .fullName)
		}
		SUT.sampleValues.forEach(doTest)
	}

	func test_extract_wrong_is_nil() {
		XCTAssertNil(SUT.extract(from: PersonaDataEntryEmailAddress.sample.embed()))
	}

	func test_variants() {
		XCTAssertEqual(SUT.Variant.allCases, [.eastern, .western])
	}

	func test_formatted_eastern() {
		XCTAssertNoDifference(
			SUT.sampleOther.description,
			"""
			Jun-fan Lee
			"Bruce"
			"""
		)
	}

	func test_formatted_wester() {
		XCTAssertNoDifference(
			SUT.sample.description,
			"""
			Bruce Wayne
			"Batman"
			"""
		)
	}

	func test_codable() throws {
		let raw = """
		{
			"variant": "western",
			"familyName": "Wayne",
			"givenNames": "Bruce",
			"nickname": "Batman"
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
