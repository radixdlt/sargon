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
		keychainService: String,
		userDefaultsSuite: String
	) -> BIOS {
		assert(_shared == nil, "Should not be created twice")
		let shared = BIOS(bundle: bundle, keychainService: keychainService, userDefaultsSuite: userDefaultsSuite)
		Self._shared = shared
		return shared
	}
}
