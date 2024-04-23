//
//  File.swift
//  
//
//  Created by Alexander Cyon on 2024-04-23.
//

import Foundation
import SargonUniFFI

#if DEBUG
extension RequestedQuantity {
	public static let sample: Self = newRequestedQuantitySample()
	public static let sampleOther: Self = newRequestedQuantitySampleOther()
}
#endif // DEBUG
