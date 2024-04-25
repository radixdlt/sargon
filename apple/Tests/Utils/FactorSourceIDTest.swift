import CustomDump
import Foundation
import Sargon
import SargonUniFFI
import XCTest

class FactorSourceIDTest<SUT_: FactorSourceIDProtocol>: Test<SUT_> {
	func test_description() {
		XCTAssertEqual(SUT.sample.description, SUT.sample.toString())
	}
}
