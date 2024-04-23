//
//  File.swift
//  
//
//  Created by Alexander Cyon on 2024-04-23.
//

import Foundation
import SargonUniFFI

#if DEBUG
extension PersonaData {
	public static let sample: Self = newPersonaDataSample()
	public static let sampleOther: Self = newPersonaDataSampleOther()
}
#endif // DEBUG
