import CustomDump
import Foundation
import Sargon
import SargonUniFFI
import XCTest

final class FactorSourceCommonTests: Test<FactorSourceCommon> {
	func test_babylon_neq_olmypia() {
		XCTAssertNotEqual(SUT.babylon(), SUT.olympia())
	}

	func test_babylon_main() {
		XCTAssertTrue(SUT.babylon(isMain: true).flags.contains(.main))
		XCTAssertFalse(SUT.babylon(isMain: false).flags.contains(.main))
	}
}
