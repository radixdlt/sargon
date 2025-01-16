import CustomDump
import Foundation
import Sargon
import SargonUniFFI
import XCTest

final class TimePeriodUnitTests: TestCase {
	func test() {
		let sut = TimePeriodUnit.days
		XCTAssertEqual(sut.values, Array(1 ... 9999))
	}
}
