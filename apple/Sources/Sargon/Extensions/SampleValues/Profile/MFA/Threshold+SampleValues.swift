import Foundation
import SargonUniFFI

#if DEBUG
extension Threshold {
	public static let sample: Self = newThresholdSample()
	public static let sampleOther: Self = newThresholdSampleOther()
}
#endif // DEBUG
