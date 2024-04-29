import CustomDump
import Foundation
import Sargon
import SargonUniFFI
import XCTest

final class FactorSourceIDTests: FactorSourceIDTest<FactorSourceID> {
	func test_as_general_is_self() {
		func doTest(_ sut: SUT) {
			XCTAssertEqual(sut.asGeneral, sut)
		}
		SUT.sampleValues.forEach(doTest)
	}
}
