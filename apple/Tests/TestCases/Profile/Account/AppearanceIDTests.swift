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
		try SUT.sampleValues.forEach(doTestCodableRoundtrip)
	}
	
	func test_from_number_of_accounts() {
		func doTest(_ count: Int, expected: SUT) {
			XCTAssertEqual(SUT.fromNumberOfAccounts(count), expected)
		}
		doTest(0, expected: SUT.sample)
		doTest(11, expected: SUT.sampleOther)
		doTest(12, expected: SUT.sample)
		doTest(23, expected: SUT.sampleOther)
	}
}
