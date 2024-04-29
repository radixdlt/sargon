import CustomDump
import Foundation
import Sargon
import SargonUniFFI
import XCTest

final class AppPreferencesTests: Test<AppPreferences> {
	func test_default_guarantee_is_99() {
		XCTAssertEqual(SUT.default.transaction.defaultDepositGuarantee, 0.99)
	}
}
