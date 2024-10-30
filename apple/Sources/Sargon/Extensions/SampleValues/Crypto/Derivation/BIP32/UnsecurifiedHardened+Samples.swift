//
//  UnsecurifiedHardened+Samples.swift
//  Sargon
//
//  Created by Alexander Cyon on 2024-10-25.
//

import SargonUniFFI

#if DEBUG
extension UnsecurifiedHardened {
    public static let sample: Self = newUnsecurifiedHardenedSample()
    public static let sampleOther: Self = newUnsecurifiedHardenedSampleOther()
}
#endif
