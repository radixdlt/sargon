//
//  File.swift
//  
//
//  Created by Alexander Cyon on 2024-04-19.
//

import Foundation
import SargonUniFFI

#if DEBUG
extension Header {
	public static let sample: Self = newHeaderSample()
	public static let sampleOther: Self = newHeaderSampleOther()
}
#endif // DEBUG
