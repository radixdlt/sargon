//
//  File.swift
//  
//
//  Created by Alexander Cyon on 2024-04-19.
//

import Foundation
import SargonUniFFI

#if DEBUG
extension MnemonicWithPassphrase {
	public static let sample: Self = newMnemonicWithPassphraseSample()
	public static let sampleOther: Self = newMnemonicWithPassphraseSampleOther()
}
#endif // DEBUG
