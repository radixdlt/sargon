import CustomDump
import Foundation
import Sargon
import SargonUniFFI
import XCTest

final class ThirdPartyDepositsTests: Test<ThirdPartyDeposits> {
	func test_default_has_some_assetsExceptionList() {
		XCTAssertNotNil(SUT.default.assetsExceptionList)
	}

	func test_default_has_some_depositorsAllowList() {
		XCTAssertNotNil(SUT.default.depositorsAllowList)
	}

	func test_isAssetsExceptionsUnknown_false() {
		XCTAssertFalse(SUT.default.isAssetsExceptionsUnknown)
	}

	func test_accountRecoveryScanned_custom_rule() {
		func doTest(_ rule: DepositRule) {
			let sut = SUT.accountRecoveryScanned(depositRule: rule)
			XCTAssertEqual(sut.depositRule, rule)
			XCTAssertTrue(sut.isAssetsExceptionsUnknown)
			XCTAssertTrue(sut.isAllowedDepositorsUnknown)
		}
		DepositRule.sampleValues.forEach(doTest)
	}

	func test_isAllowedDepositorsUnknown_false() {
		XCTAssertFalse(SUT.default.isAllowedDepositorsUnknown)
	}
}
