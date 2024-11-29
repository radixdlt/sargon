import Foundation
import SargonUniFFI

#if DEBUG
extension OnLedgerSettings {
	public static let sample: Self = newOnLedgerSettingsSample()
	public static let sampleOther: Self = newOnLedgerSettingsSampleOther()
}
#endif // DEBUG
