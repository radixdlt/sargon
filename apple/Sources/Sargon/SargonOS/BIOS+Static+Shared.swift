//
//  File.swift
//  
//
//  Created by Alexander Cyon on 2024-05-05.
//

import Foundation
import SargonUniFFI

extension BIOS {
	private nonisolated(unsafe) static var _shared: BIOS!
	
	public nonisolated(unsafe) static var shared: BIOS {
		guard let shared = Self._shared else {
			fatalError("BIOS not created, create it with `BIOS.createShared:bundle:keychainService:userDefaultsSuite`")
		}
		return shared
	}
	
	/// Can be access later with `OS.shared`
	@discardableResult
	public static func createdShared(
		bundle: Bundle,
		userDefaultsSuite: String,
		secureStorageDriver: SecureStorageDriver
	) -> BIOS {
		Self.settingShared(
			shared: BIOS(
				bundle: bundle,
				userDefaultsSuite: userDefaultsSuite,
				secureStorageDriver: secureStorageDriver
			)
		)
	}
	
	/// Can be access later with `OS.shared`
	@discardableResult
	public static func settingShared(
		shared: BIOS,
		isEmulatingFreshInstall: Bool = false
	) -> BIOS {
		if !isEmulatingFreshInstall {
			assert(_shared == nil, "Should not be created twice")
		}
		Self._shared = shared
		return shared
	}
}
