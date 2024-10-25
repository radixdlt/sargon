//
//  U30+Samples.swift
//  Sargon
//
//  Created by Alexander Cyon on 2024-10-25.
//

import SargonUniFFI

#if DEBUG
extension U30 {
    public static let sample: Self = newU30Sample()
    public static let sampleOther: Self = newU30SampleOther()
}
#endif
