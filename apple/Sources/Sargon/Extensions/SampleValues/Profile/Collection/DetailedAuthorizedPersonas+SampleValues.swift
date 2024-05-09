//
//  File.swift
//
//
//  Created by Alexander Cyon on 2024-04-22.
//

import Foundation
import SargonUniFFI

#if DEBUG
extension [AuthorizedPersonaDetailed] {
	public static let sample: Self = newDetailedAuthorizedPersonasSample()
	public static let sampleOther: Self = newDetailedAuthorizedPersonasSampleOther()
}
#endif // DEBUG
