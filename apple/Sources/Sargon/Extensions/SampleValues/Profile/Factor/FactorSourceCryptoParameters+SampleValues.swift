//
//  File.swift
//  
//
//  Created by Alexander Cyon on 2024-04-23.
//

import Foundation
import SargonUniFFI

#if DEBUG
extension FactorSourceCryptoParameters {
	public static let sample: Self = newFactorSourceCryptoParametersSample()
	public static let sampleOther: Self = newFactorSourceCryptoParametersSampleOther()
}
#endif // DEBUG
