import CustomDump
import Foundation
import Sargon
import SargonUniFFI
import XCTest

final class FactorSourceCommonTests: Test<FactorSourceCommon> {
	func test_babylon_neq_olmypia() {
		XCTAssertNotEqual(SUT.babylon(), SUT.olympia())
	}
}
