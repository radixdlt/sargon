//
//  SecurifiedU30+Samples.swift
//  Sargon
//
//  Created by Alexander Cyon on 2024-10-25.
//

import SargonUniFFI

#if DEBUG
extension SecurifiedU30 {
    public static let sample: Self = newSecurifiedSample()
    public static let sampleOther: Self = newSecurifiedSampleOther()
}
#endif
