import CustomDump
import Foundation
import Sargon
import SargonUniFFI
import XCTest


#if DEBUG
extension Drivers {
	public static func test() -> Drivers {
		Drivers(
			appVersion: "0.0.1",
			userDefaultsSuite: "works.rdx",
			secureStorageDriver: Insecure︕！TestOnly︕！Ephemeral︕！SecureStorage(
				keychainService: "test"
			)
		)
	}
	
}
#endif


final class DriversTests: TestCase {
	typealias SUT = Drivers

	func test_log_at_each_level() {
		rustLoggerLogAtEveryLevel()
	}
	
}
