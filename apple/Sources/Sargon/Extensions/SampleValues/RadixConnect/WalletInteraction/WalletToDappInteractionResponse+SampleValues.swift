import Foundation
import SargonUniFFI

#if DEBUG
extension WalletToDappInteractionResponse {
	public static let sample: Self = newWalletToDappInteractionResponseSample()
	public static let sampleOther: Self = newWalletToDappInteractionResponseSampleOther()
}
#endif // DEBUG