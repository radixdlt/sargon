//
//  File.swift
//  
//
//  Created by Alexander Cyon on 2024-04-22.
//

import Foundation
import SargonUniFFI

#if DEBUG
extension ResourceOrNonFungible {
	public static let sample: Self = newResourceOrNonFungibleSample()
	public static let sampleOther: Self = newResourceOrNonFungibleSampleOther()
}
#endif
