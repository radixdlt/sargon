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
    public var asGeneral: DerivationPath {
        self
    }
    
    public var asDerivationPath: DerivationPath { self }
}

public typealias HDPath = HdPath

extension DerivationPath {
	/// Returns the last path component
    public var lastPathComponent: HdPathComponent {
        self.path.components.last! // safe to unwrap, we disallow empty paths.
	}
  
    public var curve: SLIP10Curve {
        switch self {
        case .bip44Like: .secp256k1
        case .account, .identity: .curve25519
        }
    }

    public static func forEntity(
        kind: EntityKind,
        networkID: NetworkID,
        index: Hardened
    ) -> Self {
        switch kind {
        case .account:
            AccountPath(
                networkID: networkID,
                keyKind: .transactionSigning,
                index: index
            ).asGeneral
        case .persona:
            IdentityPath(
                networkID: networkID,
                keyKind: .transactionSigning,
                index: index
            ).asGeneral
        }
    }
}
