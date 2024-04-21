//
//  File.swift
//
//
//  Created by Alexander Cyon on 2024-04-15.
//

import Foundation
import SargonUniFFI

#if DEBUG
extension ReferencesToAuthorizedPersonas {
	public static let sample: Self = newReferencesToAuthorizedPersonasSample()
	public static let sampleOther: Self = newReferencesToAuthorizedPersonasSampleOther()
}
#endif // DEBUG
