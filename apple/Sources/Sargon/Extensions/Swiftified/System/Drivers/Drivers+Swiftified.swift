import Foundation
import SargonUniFFI

// MARK: - Drivers + @unchecked Sendable
extension Drivers: @unchecked Sendable {}

extension Drivers {
	public convenience init(
		bundle: Bundle,
		userDefaultsSuite: String,
		unsafeStorageKeyMapping: UnsafeStorageKeyMapping,
		secureStorageDriver: SecureStorageDriver
	) {
		self.init(
			appVersion: (bundle.infoDictionary?["CFBundleShortVersionString"] as? String) ?? "Unknown",
			userDefaultsSuite: userDefaultsSuite,
			unsafeStorageKeyMapping: unsafeStorageKeyMapping,
			secureStorageDriver: secureStorageDriver
		)
	}

	public convenience init(
		appVersion: String,
		userDefaultsSuite: String,
		unsafeStorageKeyMapping: UnsafeStorageKeyMapping,
		secureStorageDriver: SecureStorageDriver
	) {
		self.init(
			secureStorage: secureStorageDriver,
			hostInfo: AppleHostInfoDriver(appVersion: appVersion),
			unsafeStorage: UnsafeStorage(
				userDefaults: .init(suiteName: userDefaultsSuite)!,
				keyMapping: unsafeStorageKeyMapping
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
			profileStateChangeDriver: .shared,
            arculusCsdkDriver: ArculusCSDKDriver(),
            nfcTagDriver: NFCSessionClient()
		)
	}
}
