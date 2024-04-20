//
//  File.swift
//  
//
//  Created by Alexander Cyon on 2024-04-19.
//

import Foundation
import SargonUniFFI

extension HierarchicalDeterministicPublicKey: SargonModel {}
extension HierarchicalDeterministicPublicKey {
	public func isValidSignature(
		_ signature: Signature,
		for hashedMessage: Hash
	) -> Bool {
		publicKey.isValidSignature(signature, for: hashedMessage)
	}
}
