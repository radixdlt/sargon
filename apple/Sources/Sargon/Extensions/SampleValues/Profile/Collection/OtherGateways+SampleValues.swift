//
//  File.swift
//
//
//  Created by Alexander Cyon on 2024-04-15.
//

import Foundation
import SargonUniFFI

#if DEBUG
extension OtherGateways {
	public static let sample: Self = newOtherGatewaysSample()
	public static let sampleOther: Self = newOtherGatewaysSampleOther()
}
#endif // DEBUG