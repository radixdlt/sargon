import Foundation
import SargonUniFFI

public typealias BIOS = Bios

// MARK: - BIOS + @unchecked Sendable
extension BIOS: @unchecked Sendable {}

extension BIOS {
	public convenience init(
		bundle: Bundle,
		userDefaultsSuite: String,
		userDefaultsKeyMapping: [UnsafeStorageKey: String],
		secureStorageDriver: SecureStorageDriver
	) {
		let drivers = Drivers(
			bundle: bundle,
			userDefaultsSuite: userDefaultsSuite,
			userDefaultsKeyMapping: userDefaultsKeyMapping,
			secureStorageDriver: secureStorageDriver
		)
		// https://en.wikipedia.org/wiki/Power-on_self-test
		log.info("📬 BIOS POST (Power-On Self Test)")

		self.init(drivers: drivers)
	}
}
