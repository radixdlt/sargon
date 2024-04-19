import CustomDump
import Foundation
import Sargon
import SargonUniFFI
import XCTest

final class ComponentAddressTests: AddressTest<ComponentAddress> {
	
	func test_is_global() {
		XCTAssertTrue(SUT.sampleMainnet.isGlobal)
		XCTAssertFalse(SUT.sampleMainnetOther.isGlobal)
	}
	
	func test_is_internal() {
		XCTAssertTrue(SUT.sampleMainnetOther.isInternal)
		XCTAssertFalse(SUT.sampleMainnet.isInternal)
	}
}
