import Foundation
import SargonUniFFI

#if DEBUG
extension DerivationPath {
	public static let sample: Self = newDerivationPathSample()
	public static let sampleOther: Self = newDerivationPathSampleOther()
}
#endif // DEBUG
