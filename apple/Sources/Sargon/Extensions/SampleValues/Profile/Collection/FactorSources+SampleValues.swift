//
//  File.swift
//
//
//  Created by Alexander Cyon on 2024-04-15.
//

import Foundation
import SargonUniFFI

#if DEBUG
extension [FactorSource] {
	public static let sample: Self = newFactorSourcesSample()
	public static let sampleOther: Self = newFactorSourcesSampleOther()
}
#endif // DEBUG
