//
//  File.swift
//  
//
//  Created by Alexander Cyon on 2024-05-05.
//

import Foundation
import SargonUniFFI

extension BIOS {
	
	public nonisolated(unsafe) static var shared: BIOS {
		guard let shared = Self._shared else {
			fatalError("BIOS not created, create it with `BIOS.creatingShared:drivers`")
		}
		return shared
	}
	
	/// Can be access later with `OS.shared`
	@discardableResult
	public static func creatingShared(
		drivers: Drivers
	) -> BIOS {
		Self._creatingShared(drivers: drivers, isEmulatingFreshInstall: false)
	}
	
}

extension BIOS {
	private nonisolated(unsafe) static var _shared: BIOS!
	
	/// Can be access later with `OS.shared`
	@discardableResult
	public static func _creatingShared(
		drivers: Drivers,
		isEmulatingFreshInstall: Bool
	) -> BIOS {
		Self.settingShared(
			shared: BIOS(drivers: drivers),
			isEmulatingFreshInstall: isEmulatingFreshInstall
		)
	}
	
	/// Can be access later with `OS.shared`
	@discardableResult
	internal static func settingShared(
		shared: BIOS,
		isEmulatingFreshInstall: Bool = false
	) -> BIOS {
		if !isEmulatingFreshInstall {
			assert(_shared == nil, "Should not be created twice")
		}
		Self._shared = shared
		setRustLogLevel(.debug)
		return shared
	}
}
