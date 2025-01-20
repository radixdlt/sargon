import CustomDump
import Foundation
import Sargon
import SargonUniFFI
import XCTest

final class TimePeriodTests: Test<TimePeriod> {
	func test() {
		let sut = SUT(days: 1)
		XCTAssertEqual(sut.days(), 1)
	}
}
