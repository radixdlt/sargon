//
//  File.swift
//  
//
//  Created by Alexander Cyon on 2024-05-13.
//

import Foundation
import Sargon

#if DEBUG


extension BIOS {
	
	public convenience init(
		bundle: Bundle = .main,
		userDefaultsSuite: String = "test"
	) {
		self.init(
			bundle: bundle,
			userDefaultsSuite: userDefaultsSuite,
			secureStorageDriver: Insecure︕！TestOnly︕！Ephemeral︕！SecureStorage.init(
				keychainService: "test"
			)
		)
	}
}

#endif
