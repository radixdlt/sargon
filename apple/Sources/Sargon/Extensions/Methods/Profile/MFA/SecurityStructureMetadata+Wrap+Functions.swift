//
//  File.swift
//  
//
//  Created by Alexander Cyon on 2024-06-06.
//

import Foundation
import SargonUniFFI

extension SecurityStructureMetadata {
    public init(name: DisplayName) {
        self = newSecurityStructureMetadataNamed(name: name)
    }
}
