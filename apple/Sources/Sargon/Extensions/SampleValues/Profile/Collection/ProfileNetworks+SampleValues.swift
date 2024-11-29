import Foundation
import SargonUniFFI

#if DEBUG
extension [ProfileNetwork] {
	public static let sample: Self = newProfileNetworksSample()
	public static let sampleOther: Self = newProfileNetworksSampleOther()
}
#endif // DEBUG
