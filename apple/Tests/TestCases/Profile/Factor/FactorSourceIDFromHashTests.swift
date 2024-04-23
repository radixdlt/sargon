import CustomDump
import Foundation
import Sargon
import SargonUniFFI
import XCTest

final class FactorSourceIDFromHashTests: Test<FactorSourceIDFromHash> {
	func test_description() {
		XCTAssertEqual(SUT.sample.description, SUT.sample.toString())
	}
	
	func test_as_general() {
		XCTAssertEqual(SUT.sample.asGeneral, FactorSourceID.hash(value: SUT.sample))
	}
	
	func test_codable_roundtrip() throws {
		try SUT.sampleValues.forEach(doTestCodableRoundtrip)
	}
	
}

