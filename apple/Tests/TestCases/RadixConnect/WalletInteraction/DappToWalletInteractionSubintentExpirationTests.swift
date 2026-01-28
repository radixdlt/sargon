import Foundation
import SargonUniFFI
import XCTest

final class DappToWalletInteractionSubintentExpirationTests: TestCase {
	typealias SUT = DappToWalletInteractionSubintentExpiration

	func testGetStatus() {
		let afterDelay = SUT.afterDelay(.init(expireAfterSeconds: 100))
		XCTAssertEqual(afterDelay.getStatus(), .valid)

		let atTimeExpired = SUT.atTime(.init(unixTimestampSeconds: 0))
		XCTAssertEqual(atTimeExpired.getStatus(), .expired)

		let now = UInt64(Date.now.timeIntervalSince1970)
		let atTimeCloseToExpiration = SUT.atTime(.init(unixTimestampSeconds: now + 15))
		XCTAssertEqual(atTimeCloseToExpiration.getStatus(), .expirationTooClose)

		let atTimeValid = SUT.atTime(.init(unixTimestampSeconds: now + 60))
		XCTAssertEqual(atTimeValid.getStatus(), .valid)
	}
}
