//
//  File.swift
//  
//
//  Created by Alexander Cyon on 2024-05-05.
//

import Foundation
import SargonUniFFI

extension SargonOS {
	private nonisolated(unsafe) static var _shared: SargonOS!
	
	public nonisolated(unsafe) static var shared: SargonOS {
		guard let shared = Self._shared else {
			fatalError("OS not created, create it with `SargonOS.createdSharedBootingWith:bios`")
		}
		return shared
	}
	
	/// Can be access later with `OS.shared`
	@discardableResult
	public static func createdSharedBootingWith(bios: BIOS) async throws -> SargonOS {
		assert(_shared == nil, "Should not be created twice")
		let shared = try await SargonOS.boot(bios: bios)
		Self._shared = shared
		return shared
	}
}
