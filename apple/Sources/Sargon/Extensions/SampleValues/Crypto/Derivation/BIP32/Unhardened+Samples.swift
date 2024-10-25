//
//  Unhardened+Samples.swift
//  Sargon
//
//  Created by Alexander Cyon on 2024-10-25.
//

import SargonUniFFI

#if DEBUG
extension Unhardened {
    public static let sample: Self = newUnhardenedSample()
    public static let sampleOther: Self = newUnhardenedSampleOther()
}
#endif
