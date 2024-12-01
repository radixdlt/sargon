import Foundation
import SargonUniFFI

public typealias BIOS = Bios

// MARK: Sendable
extension BIOS: @unchecked Sendable {}

extension BIOS {
	public convenience init(
		bundle: Bundle,
		userDefaultsSuite: String,
		secureStorageDriver: SecureStorageDriver
	) {
		let drivers = Drivers(
			bundle: bundle,
			userDefaultsSuite: userDefaultsSuite,
			secureStorageDriver: secureStorageDriver
		)
		// https://en.wikipedia.org/wiki/Power-on_self-test
		log.info("📬 BIOS POST (Power-On Self Test)")

		self.init(drivers: drivers)
	}
}
