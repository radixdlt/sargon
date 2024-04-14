import CustomDump
import Foundation
import Sargon
import SargonUniFFI
import XCTest

final class NonFungibleResourceIndicatorTests: Test<NonFungibleResourceIndicator> {
	func test_ids() {
		XCTAssertFalse(SUT.sample.ids.isEmpty)
	}
}
