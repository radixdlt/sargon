//
//  File.swift
//  
//
//  Created by Alexander Cyon on 2024-05-05.
//

import Foundation
import SargonUniFFI

public final actor SwiftBIOS {
	public let bios: BIOS
	private init(bios: BIOS) {
		self.bios = bios
	}
}

extension SwiftBIOS {
	private nonisolated(unsafe) static var _shared: SwiftBIOS?
	
	public static var shared: SwiftBIOS {
		guard let shared = Self._shared else {
			fatalError("SwiftBIOS not created, create it with `SwiftBIOS.createShared:bios:`")
		}
		return shared
	}
}
	
extension SwiftBIOS {
	/// Can be access later with `SwiftBIOS.shared`
	@discardableResult
	public static func createShared(bios: BIOS) -> SwiftBIOS {
		assert(_shared == nil, "Should not be created twice")
		let shared = SwiftBIOS(bios: bios)
		Self._shared = shared
		return shared
	}

}
