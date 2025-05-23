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
			unsafeStorageKeyMapping: [:],
			secureStorageDriver: Insecure︕！TestOnly︕！Ephemeral︕！SecureStorage(
				keychainService: "test"
			)
		)
	}
}
#endif

// MARK: - DriversTests
final class DriversTests: TestCase {
	typealias SUT = Drivers

	func test_log_at_each_level() {
		rustLoggerLogAtEveryLevel()
	}

	func test_bios_insecure() async throws {
		let _ = try! await SargonOS.boot(
			bios: BIOS.insecure(),
			interactor: ThrowingHostInteractor.shared
		)
	}
}
