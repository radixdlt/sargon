//
//  File.swift
//  
//
//  Created by Alexander Cyon on 2024-04-19.
//

import Foundation
import SargonUniFFI

extension DerivationPath: SargonModel {}
extension DerivationPath: CustomStringConvertible {
    public var description: String {
        toString()
    }
}

extension DerivationPath: DerivationPathProtocol {
    public var asDerivationPath: DerivationPath { self }
}

public typealias HDPath = HdPath

extension DerivationPath {
	/// Returns the index, non hardened, so `3H` returns `3`.
	public var nonHardenedIndex: HDPathValue {
		let component = self.path.components.last! // safe to unwrap, we disallow empty paths.
		return component.nonHardenedValue
	}
  
    public var curveForScheme: SLIP10Curve {
        switch self {
        case .bip44Like: .secp256k1
        case .cap26: .curve25519
        }
    }

    public static func forEntity(
        kind: EntityKind,
        networkID: NetworkID,
        index: HDPathValue
    ) throws -> Self {
        switch kind {
        case .account:
            AccountPath(
                networkID: networkID,
                keyKind: .transactionSigning,
                index: index
            ).asDerivationPath
        case .persona:
            IdentityPath(
                networkID: networkID,
                keyKind: .transactionSigning,
                index: index
            ).asDerivationPath
        }
    }
}


extension HdPathComponent {
	public var nonHardenedValue: HDPathValue {
		hdPathComponentGetNonHardenedValue(component: self)
	}
}
