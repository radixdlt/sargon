//
//  U31+Samples.swift
//  Sargon
//
//  Created by Alexander Cyon on 2024-10-25.
//

import SargonUniFFI

#if DEBUG
extension U31 {
    public static let sample: Self = newU31Sample()
    public static let sampleOther: Self = newU31SampleOther()
}
#endif
