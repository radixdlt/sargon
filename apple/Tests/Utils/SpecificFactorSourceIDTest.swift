import CustomDump
import Foundation
import Sargon
import SargonUniFFI
import XCTest

class SpecificFactorSourceIDTest<SUT_: FactorSourceIDSpecificProtocol>: FactorSourceIDTest<SUT_> {
	func test_extract() throws {
		func doTest(_ sut: SUT) throws {
			let embedded = sut.asGeneral
			let extracted: SUT = try embedded.extract()
			XCTAssertEqual(extracted, sut)
		}
		try SUT.sampleValues.forEach(doTest)
	}

	func test_codable_roundtrip() throws {
		try SUT.sampleValues.forEach(doTestCodableRoundtrip)
	}
}
