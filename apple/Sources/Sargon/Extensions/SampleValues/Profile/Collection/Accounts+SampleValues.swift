//
//  File.swift
//  
//
//  Created by Alexander Cyon on 2024-04-15.
//

import Foundation
import SargonUniFFI

#if DEBUG
extension [Account] {
	public static let sample: Self = newAccountsSample()
	public static let sampleOther: Self = newAccountsSampleOther()
}
#endif // DEBUG
