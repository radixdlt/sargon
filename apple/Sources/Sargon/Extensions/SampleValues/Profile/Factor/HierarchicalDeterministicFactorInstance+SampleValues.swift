//
//  File.swift
//  
//
//  Created by Alexander Cyon on 2024-04-23.
//

import Foundation
import SargonUniFFI

#if DEBUG
extension HierarchicalDeterministicFactorInstance {
	public static let sample: Self = newHierarchicalDeterministicFactorInstanceSample()
	public static let sampleOther: Self = newHierarchicalDeterministicFactorInstanceSampleOther()
}
#endif // DEBUG
