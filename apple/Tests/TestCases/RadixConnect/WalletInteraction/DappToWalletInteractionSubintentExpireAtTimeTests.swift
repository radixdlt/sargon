import Foundation
import SargonUniFFI
import XCTest

final class DappToWalletInteractionSubintentExpireAtTimeTests: TestCase {
	typealias SUT = DappToWalletInteractionSubintentExpireAtTime

	func testDate() {
		var sut = SUT(unixTimestampSeconds: 0)
		XCTAssertEqual(sut.date, Date(timeIntervalSince1970: 0))

		sut = .init(unixTimestampSeconds: 500)
		XCTAssertEqual(sut.date, Date(timeIntervalSince1970: 500))

		sut = .init(unixTimestampSeconds: 1000)
		XCTAssertEqual(sut.date.timeIntervalSince1970, 1000)
	}
}
