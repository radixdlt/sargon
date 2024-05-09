import CustomDump
import Foundation
import Sargon
import SargonUniFFI
import XCTest

class SpecificFactorSourceTest<SUT_: FactorSourceProtocol>: FactorSourceTest<SUT_> {
	
	func test_extract() throws {
		func doTest(_ sut: SUT) throws {
			let embedded = sut.asGeneral
			let extracted: SUT = try embedded.extract()
			XCTAssertEqual(extracted, sut)
		}
		try SUT.sampleValues.forEach(doTest)
	}
}
