//
//  File.swift
//
//
//  Created by Alexander Cyon on 2024-04-15.
//

import Foundation
import SargonUniFFI

#if DEBUG
extension P2PLinks {
	public static let sample: Self = newP2PLinksSample()
	public static let sampleOther: Self = newP2PLinksSampleOther()
}
#endif // DEBUG
