//
//  File.swift
//
//
//  Created by Alexander Cyon on 2024-04-15.
//

import Foundation
import SargonUniFFI

#if DEBUG
extension [Persona] {
	public static let sample: Self = newPersonasSample()
	public static let sampleOther: Self = newPersonasSampleOther()
}
#endif // DEBUG
