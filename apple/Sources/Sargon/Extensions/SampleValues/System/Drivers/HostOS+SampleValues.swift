import Foundation
import SargonUniFFI

#if DEBUG
extension HostOs {
	public static let sample: Self = newHostOsSample()
	public static let sampleOther: Self = newHostOsSampleOther()
}
#endif // DEBUG

