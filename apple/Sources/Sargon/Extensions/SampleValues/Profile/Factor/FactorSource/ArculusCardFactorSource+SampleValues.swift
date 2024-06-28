//
//  File.swift
//
//
//  Created by Alexander Cyon on 2024-05-28.
//

import Foundation
import SargonUniFFI

#if DEBUG
extension ArculusCardFactorSource {
	public static let sample: Self = newArculusCardFactorSourceSample()
	
	public static let sampleOther: Self = newArculusCardFactorSourceSampleOther()
}

#endif // DEBUG
