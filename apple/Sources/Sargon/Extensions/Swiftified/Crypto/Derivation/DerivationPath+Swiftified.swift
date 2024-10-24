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

extension HdPathComponent: CustomDebugStringConvertible {
    public var debugDescription: String {
        toBIP32String()
    }
}

extension HdPathComponent: CustomStringConvertible {
    public var description: String {
        toBIP32StringDebug()
    }
}
extension HdPathComponent {
    public func toBIP32String() -> String {
        hdPathComponentToBip32String(component: self)
    }
    public func toBIP32StringDebug() -> String {
        hdPathComponentToBip32StringDebug(component: self)
    }

    public init(indexInGlobalKeySpace: UInt32) {
        self = newHdPathComponentFromGlobalKeySpace(value: indexInGlobalKeySpace)
    }
    
    public init(indexInLocalKeySpace: UInt32, keySpace: KeySpace) throws {
        self = try newHdPathComponentFromLocalKeySpace(value: indexInLocalKeySpace, keySpace: keySpace)
    }
    
    public var keySpace: KeySpace {
        hdPathComponentGetKeySpace(component: self)
    }
    
    public func indexInGlobalKeySpace() -> UInt32 {
        hdPathComponentIndexInGlobalKeySpace(component: self)
    }
    
    public func indexInLocalKeySpace() -> UInt32 {
        hdPathComponentIndexInLocalKeySpace(component: self)
	}
}
