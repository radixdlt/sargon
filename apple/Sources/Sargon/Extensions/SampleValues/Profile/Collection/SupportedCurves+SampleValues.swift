//
//  File.swift
//
//
//  Created by Alexander Cyon on 2024-04-15.
//

import Foundation
import SargonUniFFI

#if DEBUG
extension SupportedCurves {
	public static let sample: Self = newSupportedCurvesSample()
	public static let sampleOther: Self = newSupportedCurvesSampleOther()
}
#endif // DEBUG
