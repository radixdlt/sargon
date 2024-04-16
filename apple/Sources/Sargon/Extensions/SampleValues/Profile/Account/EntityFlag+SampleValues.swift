//
//  File.swift
//  
//
//  Created by Alexander Cyon on 2024-04-15.
//

import Foundation
import SargonUniFFI

#if DEBUG
extension EntityFlag {
	public static let sample: Self = newEntityFlagSample()
	public static let sampleOther: Self = newEntityFlagSampleOther()
}
#endif // DEBUG
