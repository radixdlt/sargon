//
//  File.swift
//  
//
//  Created by Matias Bzurovski on 13/8/24.
//

import Foundation
import SargonUniFFI

#if DEBUG
extension [AssetPreference] {
	public static let sample: Self = newAssetPreferencesSample()
	public static let sampleOther: Self = newAssetPreferencesSampleOther()
}
#endif // DEBUG
