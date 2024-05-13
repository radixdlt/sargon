//
//  File.swift
//  
//
//  Created by Alexander Cyon on 2024-05-03.
//

import Foundation
import SargonUniFFI


extension Drivers: @unchecked Sendable {}
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
			unsafeStorage: unsafeStorage
		)
	}

	public convenience init(
		appVersion: String,
		keychainService: String,
		userDefaultsSuite: String
	) {
		self.init(
			secureStorage: FAKE_SecureStorage_FAKE(keychainService: keychainService),
			hostInfo: HostInfo(appVersion: appVersion),
			unsafeStorage: UnsafeStorage.init(userDefaults: .init(suiteName: userDefaultsSuite)!)
		)
	}
	
	public convenience init(
		bundle: Bundle,
		keychainService: String,
		userDefaultsSuite: String
	) {
		self.init(
			appVersion: (bundle.infoDictionary?["CFBundleShortVersionString"] as? String) ?? "Unknown",
			keychainService: keychainService,
			userDefaultsSuite: userDefaultsSuite
		)
	}
}
