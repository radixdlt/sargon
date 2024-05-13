//
//  File.swift
//  
//
//  Created by Alexander Cyon on 2024-05-03.
//

import Foundation
import SargonUniFFI

public typealias BIOS = Bios
extension BIOS: @unchecked Sendable {}


extension Drivers {
	public convenience init(
		bundle: Bundle,
		userDefaultsSuite: String,
		secureStorageDriver: SecureStorageDriver
	) {
		self.init(
			appVersion: (bundle.infoDictionary?["CFBundleShortVersionString"] as? String) ?? "Unknown",
			userDefaultsSuite: userDefaultsSuite,
			secureStorageDriver: secureStorageDriver
		)
	}
	
	public convenience init(
		appVersion: String,
		userDefaultsSuite: String,
		secureStorageDriver: SecureStorageDriver
	) {
		self.init(
			secureStorage: secureStorageDriver,
			hostInfo: HostInfo(appVersion: appVersion),
			unsafeStorage: UnsafeStorage(
				userDefaults: .init(suiteName: userDefaultsSuite)!
			)
		)
	}
}

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
