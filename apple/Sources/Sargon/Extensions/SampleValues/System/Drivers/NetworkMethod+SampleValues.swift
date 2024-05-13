import Foundation
import SargonUniFFI

#if DEBUG
extension NetworkMethod {
	public static let sample: Self = newNetworkMethodSample()
	public static let sampleOther: Self = newNetworkMethodSampleOther()
}
#endif // DEBUG
