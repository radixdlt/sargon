//
//  U30+Wrap+Functions.swift
//  Sargon
//
//  Created by Alexander Cyon on 2024-10-25.
//

import SargonUniFFI


extension U30 {
    public init(value: UInt32) throws {
        self = try newU30(value: value)
    }
    public var value: UInt32 {
        u30GetValue(u30: self)
    }
}


