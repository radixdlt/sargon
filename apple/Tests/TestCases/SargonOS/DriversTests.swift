import CustomDump
import Foundation
import Sargon
import SargonUniFFI
import XCTest

extension Drivers {
	public static let test = Drivers(appVersion: "0.0.1", keychainService: "Test")
}

final class DriversTests: TestCase {
	typealias SUT = Drivers
}
