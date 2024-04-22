import CustomDump
import Foundation
import Sargon
import SargonUniFFI
import XCTest

final class AppearanceIDTests: Test<AppearanceID> {
	func test_id() {
		XCTAssertEqual(SUT.sample.id, 0)
		XCTAssertEqual(SUT.sampleOther.id, 11)
	}
	
	func test_all_cases_returns_actual_values_not_samples() {
		XCTAssertEqual(SUT.allCases.count, 12)
	}
	
	
	func test_codable_roundtrip() throws {
		try SUT.allCases.forEach(doTestCodableRoundtrip)
	}
}
