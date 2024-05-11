//
//  File.swift
//
//
//  Created by Alexander Cyon on 2024-04-15.
//

import Foundation
import SargonUniFFI

#if DEBUG
extension P2PLink {
	public static let sample: Self = newP2pLinkSample()
	public static let sampleOther: Self = newP2pLinkSampleOther()
}
#endif // DEBUG
