//
//  File.swift
//
//
//  Created by Alexander Cyon on 2024-04-15.
//

import Foundation
import SargonUniFFI

#if DEBUG
extension ProfileNetworks {
	public static let sample: Self = newProfileNetworksSample()
	public static let sampleOther: Self = newProfileNetworksSampleOther()
}
#endif // DEBUG
