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

extension BIOS {
	
	public convenience init(
		bundle: Bundle,
		keychainService: String
	) {
		let drivers = Drivers(
			bundle: bundle,
			keychainService: keychainService
		)
		
		// https://en.wikipedia.org/wiki/Power-on_self-test
		log.info("📬 BIOS POST (Power-On Self Test)")
		
		self.init(drivers: drivers)
		
	}
}
