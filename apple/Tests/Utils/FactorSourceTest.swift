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
		SUT.allCases.forEach(doTest)
	}
	
	func test_as_general_factorSourceKind() {
		func doTest(_ sut: SUT) {
			XCTAssertEqual(sut.asGeneral.factorSourceKind, sut.factorSourceKind)
		}
		SUT.allCases.forEach(doTest)
	}
}
