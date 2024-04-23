//
//  File.swift
//  
//
//  Created by Alexander Cyon on 2024-04-23.
//

import Foundation
import SargonUniFFI

#if DEBUG
extension NetworkDefinition {
	public static let sample: Self = newNetworkDefinitionSample()
	public static let sampleOther: Self = newNetworkDefinitionSampleOther()
}
#endif // DEBUG
