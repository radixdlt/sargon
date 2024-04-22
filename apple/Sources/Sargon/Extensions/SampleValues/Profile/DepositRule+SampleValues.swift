//
//  File.swift
//  
//
//  Created by Alexander Cyon on 2024-04-19.
//

import Foundation
import SargonUniFFI

#if DEBUG
extension DepositRule {
    public static let sample: Self = newDepositRuleSample()
    public static let sampleOther: Self = newDepositRuleSampleOther()
}
#endif // DEBUG
