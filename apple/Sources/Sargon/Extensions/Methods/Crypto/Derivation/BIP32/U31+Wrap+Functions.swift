//
//  U31+Wrap+Functions.swift
//  Sargon
//
//  Created by Alexander Cyon on 2024-10-25.
//

import SargonUniFFI

extension U31 {
    public init(value: UInt32) throws {
        self = try newU31(value: value)
    }
    public var value: UInt32 {
        u31GetValue(u31: self)
    }
}

