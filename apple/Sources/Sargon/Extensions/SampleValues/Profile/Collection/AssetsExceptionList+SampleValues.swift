//
//  File.swift
//  
//
//  Created by Alexander Cyon on 2024-04-22.
//

import Foundation
import SargonUniFFI

#if DEBUG
extension AssetsExceptionList {
	public static let sample: Self = newAssetsExceptionListSample()
	public static let sampleOther: Self = newAssetsExceptionListSampleOther()
}
#endif // DEBUG
