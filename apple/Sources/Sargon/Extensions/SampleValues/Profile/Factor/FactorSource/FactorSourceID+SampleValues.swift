import Foundation
import SargonUniFFI

#if DEBUG
extension FactorSourceID {
	public static let sample: Self = newFactorSourceIdSample()
	public static let sampleOther: Self = newFactorSourceIdSampleOther()
}
#endif // DEBUG
