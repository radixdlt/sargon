//
//  File.swift
//
//
//  Created by Alexander Cyon on 2024-04-19.
//

import Foundation
import SargonUniFFI

#if DEBUG
extension MatrixOfFactorSources {
    public static let sample: Self = newMatrixOfFactorSourcesSample()
    public static let sampleOther: Self = newMatrixOfFactorSourcesSampleOther()
}
#endif // DEBUG
