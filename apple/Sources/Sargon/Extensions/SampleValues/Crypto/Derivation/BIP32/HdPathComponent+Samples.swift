//
//  HdPathComponent+Samples.swift
//  Sargon
//
//  Created by Alexander Cyon on 2024-10-25.
//
import SargonUniFFI
#if DEBUG
extension HdPathComponent {
    public static let sample: Self = newHdPathComponentSample()
    public static let sampleOther: Self = newHdPathComponentSampleOther()
}
#endif
