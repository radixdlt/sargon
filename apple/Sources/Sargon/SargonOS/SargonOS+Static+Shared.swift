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
	internal nonisolated(unsafe) static var _shared: SargonOS!
	
	public nonisolated(unsafe) static var shared: SargonOS {
		guard let shared = Self._shared else {
			fatalError("OS not created, create it with `SargonOS.createdSharedBootingWith:bios`")
		}
		return shared
	}
	
	/// Can be access later with `OS.shared`
	@discardableResult
	public static func createdSharedBootingWith(
		bios: BIOS,
		isEmulatingFreshInstall: Bool = false
	) async throws -> SargonOS {
		if !isEmulatingFreshInstall, _shared != nil {
			throw SargonOSAlreadyBooted()
		}
		let shared = try await SargonOS.boot(bios: bios)
		Self._shared = shared
		return shared
	}
}
