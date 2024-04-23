import CustomDump
import Foundation
import Sargon
import SargonUniFFI
import XCTest

final class RequestedQuantityTests: Test<RequestedQuantity> {
	
	func test_exactly() {
		XCTAssertEqual(SUT.exactly(1), SUT.sample)
	}
	
	func test_atLeast() {
		XCTAssertEqual(SUT.atLeast(1), SUT.sampleOther)
	}
	
	func test_isValid_true() {
		XCTAssertTrue(SUT.sample.isValid)
	}
	
	func test_isValid_false() {
		let sut = SUT(
			quantifier: .exactly,
			quantity: 0
		)
		XCTAssertFalse(sut.isValid)
	}
	
	func test_is_fulfilled_by_true() {
		XCTAssertTrue(SUT.atLeast(1).isFulfilled(by: 1))
	}
	
	func test_is_fulfilled_by_false() {
		XCTAssertFalse(SUT.atLeast(2).isFulfilled(by: 1))
		XCTAssertFalse(SUT.exactly(2).isFulfilled(by: 3))
	}
	
	func test_codable() throws {
		let raw = """
		{
			"quantifier": "exactly",
			"quantity": 1
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
		try SUT.sampleValues.forEach(doTestCodableRoundtrip)
	}
}
