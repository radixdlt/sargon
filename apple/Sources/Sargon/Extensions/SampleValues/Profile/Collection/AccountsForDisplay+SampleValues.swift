//
//  File.swift
//
//
//  Created by Alexander Cyon on 2024-04-22.
//

import Foundation
import SargonUniFFI

#if DEBUG
extension AccountsForDisplay {
	public static let sample: Self = newAccountsForDisplaySample()
	public static let sampleOther: Self = newAccountsForDisplaySampleOther()
}
#endif // DEBUG
