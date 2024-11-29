import Foundation
import SargonUniFFI

#if DEBUG
extension ProfileNetwork {
	public static let sample: Self = newProfileNetworkSample()
	public static let sampleOther: Self = newProfileNetworkSampleOther()
}
#endif
