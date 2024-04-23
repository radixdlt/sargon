//
//  File.swift
//  
//
//  Created by Alexander Cyon on 2024-04-22.
//

import Foundation
import SargonUniFFI

#if DEBUG
extension AssetException {
	public static let sample: Self = newAssetExceptionSample()
	public static let sampleOther: Self = newAssetExceptionSampleOther()
}
#endif // DEBUG
