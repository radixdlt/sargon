//
//  File.swift
//  
//
//  Created by Alexander Cyon on 2024-04-19.
//

import Foundation
import SargonUniFFI

extension DerivationPath {
    public var path: HDPath {
        derivationPathToHdPath(path: self)
    }
    
    public func toString() -> String {
        derivationPathToString(path: self)
    }
    
    public init(string: String) throws {
        self = try newDerivationPathFromString(string: string)
    }

}
