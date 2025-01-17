import Foundation
import SargonUniFFI

#if DEBUG
extension TimePeriod {
	public static let sample: Self = newTimePeriodSample()
	public static let sampleOther: Self = newTimePeriodSampleOther()
}
#endif // DEBUG
