//
//  File.swift
//  
//
//  Created by Alexander Cyon on 2024-04-22.
//

import Foundation
import SargonUniFFI

#if DEBUG
extension LedgerHardwareWalletModel {
    public static let sample: Self = newLedgerHwWalletModelSample()
    public static let sampleOther: Self = newLedgerHwWalletModelSampleOther()
}
#endif // DEBUG
