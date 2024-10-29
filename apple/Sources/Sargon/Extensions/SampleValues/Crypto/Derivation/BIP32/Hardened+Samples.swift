//
//  Hardened+Samples.swift
//  Sargon
//
//  Created by Alexander Cyon on 2024-10-25.
//

import SargonUniFFI


#if DEBUG
extension Hardened {
    public static let sample: Self = newHardenedSample()
    public static let sampleOther: Self = newHardenedSampleOther()
}
#endif
