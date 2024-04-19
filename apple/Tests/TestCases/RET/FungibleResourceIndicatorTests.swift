import CustomDump
import Foundation
import Sargon
import SargonUniFFI
import XCTest

final class FungibleResourceIndicatorTests: Test<FungibleResourceIndicator> {
	func test_amount() {
		XCTAssertEqual(SUT.sample.amount, 1)
	}
}
