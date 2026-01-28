import Foundation
import SargonUniFFI
import XCTest

final class InstantTests: TestCase {
	typealias SUT = Instant

	func testDate() {
		var sut = SUT(secondsSinceUnixEpoch: 0)
		XCTAssertEqual(sut.date, Date(timeIntervalSince1970: 0))

		sut = .init(secondsSinceUnixEpoch: 500)
		XCTAssertEqual(sut.date, Date(timeIntervalSince1970: 500))

		sut = .init(secondsSinceUnixEpoch: 1000)
		XCTAssertEqual(sut.date.timeIntervalSince1970, 1000)
	}
}
