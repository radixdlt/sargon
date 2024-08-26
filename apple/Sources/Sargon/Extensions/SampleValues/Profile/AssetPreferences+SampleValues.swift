//
//  File.swift
//  
//
//  Created by Matias Bzurovski on 13/8/24.
//

import Foundation
import SargonUniFFI

#if DEBUG
extension [ResourceAppPreference] {
	public static let sample: Self = newResourcePreferencesSample()
	public static let sampleOther: Self = newResourcePreferencesSampleOther()
}
#endif // DEBUG
