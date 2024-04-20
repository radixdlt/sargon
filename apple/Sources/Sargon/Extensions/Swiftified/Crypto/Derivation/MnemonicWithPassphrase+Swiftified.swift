//
//  File.swift
//  
//
//  Created by Alexander Cyon on 2024-04-19.
//

import Foundation
import SargonUniFFI

extension MnemonicWithPassphrase: SargonModel {}
extension MnemonicWithPassphrase: SargonObjectCodable {}

extension MnemonicWithPassphrase {
	public func derivePublicKey(
		path: some DerivationPathProtocol
	) -> HierarchicalDeterministicPublicKey {
		derivePublicKeys(paths: [path]).first!
	}
}
