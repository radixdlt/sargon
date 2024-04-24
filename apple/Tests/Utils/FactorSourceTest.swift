import CustomDump
import Foundation
import Sargon
import SargonUniFFI
import XCTest

class SpecificFactorSourceTest<SUT_: FactorSourceSpecificProtocol>: FactorSourceTest<SUT_> {
	
	func test_extract() throws {
		func doTest(_ sut: SUT) throws {
			let embedded = sut.asGeneral
			let extracted: SUT = try embedded.extract()
			XCTAssertEqual(extracted, sut)
		}
		try SUT.sampleValues.forEach(doTest)
	}
}

class FactorSourceTest<SUT_: FactorSourceProtocol>: Test<SUT_> {
	
	func test_as_general_factorSourceID() {
		func doTest(_ sut: SUT) {
			XCTAssertEqual(sut.asGeneral.factorSourceID, sut.factorSourceID)
		}
		SUT.sampleValues.forEach(doTest)
	}
	
	func test_as_general_factorSourceKind() {
		func doTest(_ sut: SUT) {
			XCTAssertEqual(sut.asGeneral.kind, sut.kind)
		}
		SUT.sampleValues.forEach(doTest)
	}
}
