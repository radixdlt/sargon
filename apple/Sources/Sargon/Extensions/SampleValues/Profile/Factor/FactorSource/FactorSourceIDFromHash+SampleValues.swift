import Foundation
import SargonUniFFI

#if DEBUG
extension FactorSourceIDFromHash {
	public static let sample: Self = newFactorSourceIdFromHashSample()
	public static let sampleOther: Self = newFactorSourceIdFromHashSampleOther()
}
#endif // DEBUG
