import CustomDump
import Foundation
import Sargon
import SargonUniFFI
import XCTest

class SpecificFactorSourceTest<SUT_: FactorSourceProtocol>: FactorSourceTest<SUT_> {
	
	func test_extract() throws {
		try eachSample { sut in
			let embedded = sut.asGeneral
			let extracted: SUT = try embedded.extract()
			XCTAssertEqual(extracted, sut)
		}
	}
}
