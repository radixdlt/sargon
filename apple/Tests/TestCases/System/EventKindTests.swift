import CustomDump
import Foundation
import Sargon
import SargonUniFFI
import XCTest

final class EventKindTests: Test<EventKind> {
	func test_all_cases() {
		XCTAssertGreaterThan(SUT.allCases.count, 1)
	}
	
	func test_affectsCurrentAccounts() {
		func doTest(_ sut: SUT, expected: Bool) {
			XCTAssertEqual(sut.affectsCurrentAccounts, expected)
		}
		doTest(.accountAdded, expected: true)
		doTest(.accountUpdated, expected: true)
		doTest(.profileLastUsedOnOtherDevice, expected: false)
	}
	
	func test_affectsCurrentNetwork() {
		func doTest(_ sut: SUT, expected: Bool) {
			XCTAssertEqual(sut.affectsCurrentNetwork, expected)
		}
		doTest(.accountAdded, expected: false)
		doTest(.gatewayChangedCurrent, expected: true)
		doTest(.profileImported, expected: true)
	}
	
	func test_affectsSavedGateways() {
		func doTest(_ sut: SUT, expected: Bool) {
			XCTAssertEqual(sut.affectsSavedGateways, expected)
		}
		doTest(.accountAdded, expected: false)
		doTest(.profileLastUsedOnOtherDevice, expected: false)
		doTest(.gatewayChangedCurrent, expected: true)
	}
}
