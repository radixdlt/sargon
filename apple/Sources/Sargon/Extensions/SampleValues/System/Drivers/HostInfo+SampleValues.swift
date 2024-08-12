import Foundation
import SargonUniFFI

#if DEBUG
extension HostInfo {
	public static let sample: Self = newHostInfoSample()
	public static let sampleOther: Self = newHostInfoSampleOther()
}
#endif // DEBUG
