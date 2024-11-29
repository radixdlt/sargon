import Foundation
import SargonUniFFI

#if DEBUG
extension [ResourceOrNonFungible] {
	public static let sample: Self = newDepositorsAllowListSample()
	public static let sampleOther: Self = newDepositorsAllowListSampleOther()
}
#endif // DEBUG
