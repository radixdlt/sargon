import Foundation
import SargonUniFFI

#if DEBUG
extension MatrixOfFactorSources {
	public static let sample: Self = newMatrixOfFactorSourcesSample()
	public static let sampleOther: Self = newMatrixOfFactorSourcesSampleOther()
}
#endif // DEBUG
