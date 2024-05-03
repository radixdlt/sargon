import CustomDump
import Foundation
import Sargon
import SargonUniFFI
import XCTest

extension BIOS {
	static let test = BIOS(bundle: .main, keychainService: "Test")
}

final class BIOSTests: TestCase {
	typealias SUT = BIOS
}
