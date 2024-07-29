import Foundation
import SargonUniFFI

#if DEBUG
extension FactorSourceCommon {
	public static let sample: Self = newFactorSourceCommonSample()
	public static let sampleOther: Self = newFactorSourceCommonSampleOther()
}
#endif // DEBUG
