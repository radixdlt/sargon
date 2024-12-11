import Foundation
import SargonUniFFI

// MARK: - Drivers + @unchecked Sendable
extension Drivers: @unchecked Sendable {}

extension Drivers {
	public convenience init(
		bundle: Bundle,
		userDefaultsSuite: String,
		userDefaultsKeyMapping: [UnsafeStorageKey: String],
		secureStorageDriver: SecureStorageDriver
	) {
		self.init(
			appVersion: (bundle.infoDictionary?["CFBundleShortVersionString"] as? String) ?? "Unknown",
			userDefaultsSuite: userDefaultsSuite,
			userDefaultsKeyMapping: userDefaultsKeyMapping,
			secureStorageDriver: secureStorageDriver
		)
	}

	public convenience init(
		appVersion: String,
		userDefaultsSuite: String,
		userDefaultsKeyMapping: [UnsafeStorageKey: String],
		secureStorageDriver: SecureStorageDriver
	) {
		self.init(
			secureStorage: secureStorageDriver,
			hostInfo: AppleHostInfoDriver(appVersion: appVersion),
			unsafeStorage: UnsafeStorage(
				userDefaults: .init(suiteName: userDefaultsSuite)!,
				keyMapping: userDefaultsKeyMapping
			)
		)
	}
}

extension Drivers {
	public convenience init(
		secureStorage: SecureStorageDriver,
		hostInfo: HostInfoDriver,
		unsafeStorage: UnsafeStorage
	) {
		self.init(
			networking: .shared,
			secureStorage: secureStorage,
			entropyProvider: .shared,
			hostInfo: hostInfo,
			logging: .shared,
			eventBus: .shared,
			fileSystem: .shared,
			unsafeStorage: unsafeStorage,
			profileStateChangeDriver: .shared
		)
	}
}
