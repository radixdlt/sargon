import CustomDump
import Foundation
import Sargon
import SargonUniFFI
import XCTest

final class FactorSourceIDFromAddressTests: Test<FactorSourceIDFromAddress> {
	func test_description() {
		XCTAssertEqual(SUT.sample.description, SUT.sample.toString())
	}
	
	func test_as_general() {
		XCTAssertEqual(SUT.sample.asGeneral, FactorSourceID.address(value: SUT.sample))
	}
	
	func test_codable_roundtrip() throws {
		try SUT.allCases.forEach(doTestCodableRoundtrip)
	}
	
}

