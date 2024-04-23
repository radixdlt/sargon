import CustomDump
import Foundation
import Sargon
import SargonUniFFI
import XCTest

class FactorSourceTest<SUT_: FactorSourceProtocol>: Test<SUT_> {
	
	func test_as_general_factorSourceID() {
		func doTest(_ sut: SUT) {
			XCTAssertEqual(sut.asGeneral.factorSourceID, sut.factorSourceID)
		}
		SUT.sampleValues.forEach(doTest)
	}
	
	func test_as_general_factorSourceKind() {
		func doTest(_ sut: SUT) {
			XCTAssertEqual(sut.asGeneral.factorSourceKind, sut.factorSourceKind)
		}
		SUT.sampleValues.forEach(doTest)
	}
}
