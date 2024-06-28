//
//  File.swift
//
//
//  Created by Alexander Cyon on 2024-04-19.
//

import Foundation
import SargonUniFFI

#if DEBUG
extension SecurityStructureMetadata {
    public static let sample: Self = newSecurityStructureMetadataSample()
    public static let sampleOther: Self = newSecurityStructureMetadataSampleOther()
}
#endif // DEBUG
