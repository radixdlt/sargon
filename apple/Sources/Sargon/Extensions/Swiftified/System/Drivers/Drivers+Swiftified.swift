import Foundation
import SargonUniFFI

// MARK: - Drivers + @unchecked Sendable
extension Drivers: @unchecked Sendable {}

extension Drivers {
	public convenience init(
		bundle: Bundle,
		userDefaultsSuite: String,
		unsafeStorageKeyMapping: UnsafeStorageKeyMapping,
		secureStorageDriver: SecureStorageDriver,
        arculuCSDKDriver: ArculusCsdkDriver,
        nftTagDriver: NfcTagDriver
	) {
		self.init(
			appVersion: (bundle.infoDictionary?["CFBundleShortVersionString"] as? String) ?? "Unknown",
			userDefaultsSuite: userDefaultsSuite,
			unsafeStorageKeyMapping: unsafeStorageKeyMapping,
			secureStorageDriver: secureStorageDriver,
            arculuCSDKDriver: arculuCSDKDriver,
            nftTagDriver: nftTagDriver
		)
	}

	public convenience init(
		appVersion: String,
		userDefaultsSuite: String,
		unsafeStorageKeyMapping: UnsafeStorageKeyMapping,
		secureStorageDriver: SecureStorageDriver,
        arculuCSDKDriver: ArculusCsdkDriver,
        nftTagDriver: NfcTagDriver
	) {
		self.init(
			secureStorage: secureStorageDriver,
			hostInfo: AppleHostInfoDriver(appVersion: appVersion),
			unsafeStorage: UnsafeStorage(
				userDefaults: .init(suiteName: userDefaultsSuite)!,
				keyMapping: unsafeStorageKeyMapping
			),
            arculusCSDKDriver: arculuCSDKDriver,
            nftTagDriver: nftTagDriver
		)
	}
}

extension Drivers {
	public convenience init(
		secureStorage: SecureStorageDriver,
		hostInfo: HostInfoDriver,
		unsafeStorage: UnsafeStorage,
        arculusCSDKDriver: ArculusCsdkDriver,
        nftTagDriver: NfcTagDriver
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
            arculusCsdkDriver: arculusCSDKDriver,
            nfcTagDriver: nftTagDriver
		)
	}
}
