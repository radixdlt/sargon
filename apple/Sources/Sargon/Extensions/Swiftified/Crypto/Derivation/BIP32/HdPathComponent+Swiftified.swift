//
//  HdPathComponent+Swiftified.swift
//  Sargon
//
//  Created by Alexander Cyon on 2024-10-25.
//

import SargonUniFFI

extension HdPathComponent: BaseHDPathComponentProtocol {}


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
    public func asHardened() throws -> Hardened {
        try hdPathComponentToHardened(component: self)
    }
}
