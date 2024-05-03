import CustomDump
import Foundation
import Sargon
import SargonUniFFI
import XCTest

final class BIOSTests: TestCase {
	typealias SUT = BIOS

	func test_bios_post() {
		let _ = SUT(bundle: .main, keychainService: "test")
	}
}
