import Foundation
import SargonUniFFI

#if DEBUG
extension DepositorsAllowList {
	public static let sample: Self = newDepositorsAllowListSample()
	public static let sampleOther: Self = newDepositorsAllowListSampleOther()
}
#endif // DEBUG
