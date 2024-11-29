import Foundation
import SargonUniFFI

#if DEBUG
extension DepositRule {
	public static let sample: Self = newDepositRuleSample()
	public static let sampleOther: Self = newDepositRuleSampleOther()
}
#endif // DEBUG
