import CustomDump
import Foundation
import Sargon
import SargonUniFFI
import XCTest

class RefTest<SUT_: SargonReferenceType>: Test<SUT_> {
	func test_take_twice_throws() {
		func doTest(_ sut: SUT) {
			XCTAssertNoThrow(try sut.take())
			XCTAssertThrowsError(try sut.take())
		}
		SUT.sampleValues.forEach(doTest)
	}
	
	func test_inner_take_roundtrip() {
		func doTest(_ inner: SUT.Inner) {
			let sut = SUT.from(inner: inner)
			try XCTAssertEqual(sut.take(), inner)
		}
		SUT.Inner.sampleValues.forEach(doTest)
	}
}
