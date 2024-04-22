//
//  File.swift
//  
//
//  Created by Alexander Cyon on 2024-04-19.
//

import Foundation
import SargonUniFFI

#if DEBUG
extension FiatCurrency {
	public static let sample: Self = newFiatCurrencySample()
	public static let sampleOther: Self = newFiatCurrencySampleOther()
}
#endif // DEBUG
