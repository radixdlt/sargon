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
}
