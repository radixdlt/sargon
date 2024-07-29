//
//  File.swift
//  
//
//  Created by Alexander Cyon on 2024-05-28.
//

import Foundation
import SargonUniFFI

#if DEBUG
extension OffDeviceMnemonicFactorSource {
	public static let sample: Self = newOffDeviceMnemonicFactorSourceSample()
	
	public static let sampleOther: Self = newOffDeviceMnemonicFactorSourceSampleOther()
}

#endif // DEBUG
