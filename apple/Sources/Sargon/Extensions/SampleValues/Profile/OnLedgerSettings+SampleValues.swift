//
//  File.swift
//  
//
//  Created by Alexander Cyon on 2024-04-20.
//

import Foundation
import SargonUniFFI

#if DEBUG
extension OnLedgerSettings {
	public static let sample: Self = newOnLedgerSettingsSample()
	public static let sampleOther: Self = newOnLedgerSettingsSampleOther()
}
#endif // DEBUG
