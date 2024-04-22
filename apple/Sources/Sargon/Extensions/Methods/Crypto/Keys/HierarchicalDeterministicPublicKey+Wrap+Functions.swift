//
//  File.swift
//  
//
//  Created by Alexander Cyon on 2024-04-20.
//

import Foundation
import SargonUniFFI

extension HierarchicalDeterministicPublicKey {
	public func isValidSignature(
		_ intoSignature: IntoSignatureProtocol,
		for hashedMessage: Hash
	) -> Bool {
		hierarchicalDeterministicPublicKeyIsValidSignatureForHash(
			key: self,
			signature: intoSignature.signature,
			hash: hashedMessage
		)
	}
}
