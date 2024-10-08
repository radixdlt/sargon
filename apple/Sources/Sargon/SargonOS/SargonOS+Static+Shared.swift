//
//  File.swift
//  
//
//  Created by Alexander Cyon on 2024-05-05.
//

import Foundation
import SargonUniFFI

struct SargonOSAlreadyBooted: LocalizedError {
	var errorDescription: String? {
		"Radix Wallet core already initialized, should not have been initialized twice. This is a Radix developer error."
	}
}

extension SargonOS {
	
	public nonisolated(unsafe) static var shared: SargonOS {
		guard let shared = Self._shared else {
			fatalError("`SargonOS.shared` not created, create it with `SargonOS.creatingShared:bootingWith` and pass it a `BIOS`.")
		}
		return shared
	}
	
	/// Can be access later with `OS.shared`
	@discardableResult
	public static func creatingShared(
		bootingWith bios: BIOS
	) async throws -> SargonOS {
		try await _creatingShared(
			bootingWith: bios,
			isEmulatingFreshInstall: false
		)
	}
	
}

extension SargonOS {
	
	/// Can be access later with `OS.shared`
	@discardableResult
	internal static func _creatingShared(
		bootingWith bios: BIOS,
		isEmulatingFreshInstall: Bool
	) async throws -> SargonOS {
		if !isEmulatingFreshInstall, _shared != nil {
			throw SargonOSAlreadyBooted()
		}
		let shared = await SargonOS.boot(bios: bios)
		Self._shared = shared
		return shared
	}
	
	internal nonisolated(unsafe) static var _shared: SargonOS!
	
}

