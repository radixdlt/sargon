import Foundation
import Sargon
import SargonUniFFI
import XCTest

final class HostOSTests: Test<HostOs> {
	func testGetName() {
		let sut = SUT.sample

		XCTAssertEqual(sut.name(), "iOS")
	}

	func testGetVendor() {
		let sut = SUT.sample

		XCTAssertEqual(sut.vendor(), "Apple")
	}

	func testGetVersion() {
		let sut = SUT.sample

		XCTAssertEqual(sut.version(), "iOS 17.4.1")
	}
}
