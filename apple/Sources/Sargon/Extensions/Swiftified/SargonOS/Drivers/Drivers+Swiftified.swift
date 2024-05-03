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
		hostInfo: HostInfoDriver
	) {
		self.init(
			networking: .shared,
			secureStorage: secureStorage,
			entropyProvider: .shared,
			hostInfo: hostInfo,
			loggingDriver: .shared
		)
	}

	public convenience init(
		appVersion: String,
		keychainService: String
	) {
		self.init(
			secureStorage: UnsafeMockSecureStorage(keychainService: keychainService),
			hostInfo: HostInfo(appVersion: appVersion)
		)
	}
	
	public convenience init(
		bundle: Bundle,
		keychainService: String
	) {
		self.init(
			appVersion: (bundle.infoDictionary?["CFBundleShortVersionString"] as? String) ?? "Unknown",
			keychainService: keychainService
		)
	}
}
