//
//  File.swift
//
//
//  Created by Alexander Cyon on 2024-05-03.
//

import Foundation
import SargonUniFFI

extension EntropyProviderDriver where Self == EntropyProvider {
	public static var shared: Self { Self.shared }
}

public final actor EntropyProvider {
	internal init() {}
	public static let shared = EntropyProvider()
}

extension EntropyProvider: EntropyProviderDriver {
	nonisolated public func generateSecureRandomBytes() -> Entropy32Bytes {
		Entropy32Bytes.generate()
	}
}
