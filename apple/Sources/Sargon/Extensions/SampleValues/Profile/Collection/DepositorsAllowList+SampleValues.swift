//
//  File.swift
//  
//
//  Created by Alexander Cyon on 2024-04-22.
//

import Foundation
import SargonUniFFI

#if DEBUG
extension DepositorsAllowList {
	public static let sample: Self = newDepositorsAllowListSample()
	public static let sampleOther: Self = newDepositorsAllowListSampleOther()
}
#endif // DEBUG
