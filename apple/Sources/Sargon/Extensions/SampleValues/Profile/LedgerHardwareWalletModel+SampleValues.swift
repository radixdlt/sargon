import Foundation
import SargonUniFFI

#if DEBUG
extension LedgerHardwareWalletModel {
	public static let sample: Self = newLedgerHwWalletModelSample()
	public static let sampleOther: Self = newLedgerHwWalletModelSampleOther()
}
#endif // DEBUG
