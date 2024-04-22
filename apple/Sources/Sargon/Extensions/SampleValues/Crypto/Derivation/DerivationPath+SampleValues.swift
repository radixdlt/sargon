//
//  File.swift
//  
//
//  Created by Alexander Cyon on 2024-04-19.
//

import Foundation
import SargonUniFFI

#if DEBUG
extension DerivationPath {
    public static let sample: Self = newDerivationPathSample()
    public static let sampleOther: Self = newDerivationPathSampleOther()
}
#endif // DEBUG

