import CustomDump
import Foundation
import Sargon
import SargonUniFFI
import XCTest

final class FactorSourceIDFromAddressTests: SpecificFactorSourceIDTest<FactorSourceIDFromAddress> {
	func test_as_general() {
		XCTAssertEqual(SUT.sample.asGeneral, FactorSourceID.address(value: SUT.sample))
	}

	func test_extract_wrong_throws() {
		func doTest(_ sut: SUT) {
			XCTAssertThrowsError(try sut.asGeneral.extract(as: FactorSourceIDFromHash.self))
		}
		SUT.sampleValues.forEach(doTest)
	}
}
