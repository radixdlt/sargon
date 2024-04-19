//
//  File.swift
//  
//
//  Created by Alexander Cyon on 2024-04-19.
//

import Foundation
import SargonUniFFI

#if DEBUG
extension HierarchicalDeterministicPublicKey {
    public static let sample: Self = newHierarchicalDeterministicPublicKeySample()
    public static let sampleOther: Self = newHierarchicalDeterministicPublicKeySampleOther()
}
#endif // DEBUG

