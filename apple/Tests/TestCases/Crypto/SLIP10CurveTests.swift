import CustomDump
import Foundation
import Sargon
import SargonUniFFI
import XCTest

final class SLIP10CurveTests: Test<SLIP10Curve> {
	func test_description() {
		XCTAssertNoDifference(SUT.sample.toString(), SUT.sample.description)
		XCTAssertNoDifference(SUT.sampleOther.toString(), SUT.sampleOther.description)
	}
	
	func test_string_roundtrip() throws {
		func doTest(_ sut: SUT) throws {
			let string = sut.toString()
			let fromString = try XCTUnwrap(SUT.init(rawValue: string))
			XCTAssertEqual(fromString, sut)
		}
		try SUT.allCases.forEach(doTest)
	}
	
	func test_codable_roundtrip() throws {
		try SUT.allCases.forEach(doTestCodableRoundtrip)
	}
}
