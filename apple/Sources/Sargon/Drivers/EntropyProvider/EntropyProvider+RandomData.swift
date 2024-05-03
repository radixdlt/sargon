//
//  File.swift
//
//
//  Created by Alexander Cyon on 2024-05-03.
//

import Foundation
import SargonUniFFI

public final actor EntropyProvider {
	private init() {}
	public static let shared = EntropyProvider()
}
extension EntropyProvider: EntropyProviderDriver {
	nonisolated public func generateSecureRandomBytes() -> Entropy32Bytes {
		Entropy32Bytes.generate()
	}
}
