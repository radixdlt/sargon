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
	
	public static func insecure(
		bundle: Bundle = .main,
		userDefaultsSuite: String = "test"
	) -> BIOS {
		BIOS(
			bundle: bundle,
			userDefaultsSuite: userDefaultsSuite,
			secureStorageDriver: Insecure︕！TestOnly︕！Ephemeral︕！SecureStorage.init(
				keychainService: "test"
			)
		)
	}
}

#endif
