//
//  File.swift
//
//
//  Created by Alexander Cyon on 2024-04-22.
//

import Foundation
import SargonUniFFI

#if DEBUG
extension TrustedContactFactorSource {
	public static let sample: Self = newTrustedContactFactorSourceSample()
	public static let sampleOther: Self = newTrustedContactFactorSourceSampleOther()
}
#endif // DEBUG
