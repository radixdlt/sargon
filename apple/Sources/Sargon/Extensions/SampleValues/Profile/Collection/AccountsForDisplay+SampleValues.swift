import Foundation
import SargonUniFFI

#if DEBUG
extension [AccountForDisplay] {
	public static let sample: Self = newAccountsForDisplaySample()
	public static let sampleOther: Self = newAccountsForDisplaySampleOther()
}
#endif // DEBUG
