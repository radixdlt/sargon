//
//  File.swift
//  
//
//  Created by Alexander Cyon on 2024-04-22.
//

import Foundation
import SargonUniFFI

extension FactorSourceKind: SargonModel {}
extension FactorSourceKind: CustomStringConvertible {
    public var description: String {
        toString()
    }
}

extension FactorSourceKind {
    
    public var rawValue: String {
        toString()
    }

    public init?(rawValue: String) {
        try? self.init(string: rawValue)
    }
}
