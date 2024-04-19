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
