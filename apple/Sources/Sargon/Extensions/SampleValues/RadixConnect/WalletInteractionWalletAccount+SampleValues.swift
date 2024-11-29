import Foundation
import SargonUniFFI

#if DEBUG
extension WalletInteractionWalletAccount {
	public static let sample: Self = newWalletInteractionWalletAccountSample()
	public static let sampleOther: Self = newWalletInteractionWalletAccountSampleOther()
}
#endif // DEBUG
