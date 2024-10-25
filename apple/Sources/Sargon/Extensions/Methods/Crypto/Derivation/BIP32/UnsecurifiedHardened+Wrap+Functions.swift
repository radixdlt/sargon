//
//  UnsecurifiedHardened+Wrap+Functions.swift
//  Sargon
//
//  Created by Alexander Cyon on 2024-10-25.
//

import SargonUniFFI

extension UnsecurifiedHardened {
   
    public static let globalOffset: UInt32 = bip32ConstantGlobalOffsetHardened()
    
    public init(u30: U30) {
        self = newUnsecurifiedHardened(u30: u30)
    }

    public init(localKeySpace: UInt32) throws {
        self = try newUnsecurifiedHardenedFromLocalKeySpace(value: localKeySpace)
    }
    
    public init(globalKeySpace: UInt32) throws {
        self = try newUnsecurifiedHardenedFromGlobalKeySpace(value: globalKeySpace)
    }

    public func indexInLocalKeySpace() -> UInt32 {
        unsecurifiedHardenedIndexInLocalKeySpace(unsecurifiedHardened: self)
    }

    public func indexInGlobalKeySpace() -> UInt32 {
        unsecurifiedHardenedIndexInGlobalKeySpace(unsecurifiedHardened: self)
    }
}
