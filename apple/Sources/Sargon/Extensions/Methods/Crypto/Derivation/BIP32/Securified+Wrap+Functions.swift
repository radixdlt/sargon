//
//  Securified+Wrap+Functions.swift
//  Sargon
//
//  Created by Alexander Cyon on 2024-10-25.
//

import SargonUniFFI


extension SecurifiedU30 {
    public static let globalOffset: UInt32 = bip32ConstantGlobalOffsetSecurified()
    
    public init(u30: U30) {
        self = newSecurified(u30: u30)
    }
    
    public init(localKeySpace: UInt32) throws {
        self = try newSecurifiedFromLocalKeySpace(value: localKeySpace)
    }
    
    public init(globalKeySpace: UInt32) throws {
        self = try newSecurifiedFromGlobalKeySpace(value: globalKeySpace)
    }
    
    public func indexInLocalKeySpace() -> UInt32 {
        securifiedIndexInLocalKeySpace(securified: self)
    }
    
    public func indexInGlobalKeySpace() -> UInt32 {
        securifiedIndexInGlobalKeySpace(securified: self)
    }
}
