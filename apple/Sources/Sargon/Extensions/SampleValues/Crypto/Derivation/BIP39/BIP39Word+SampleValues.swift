import Foundation
import SargonUniFFI

#if DEBUG
extension BIP39Word {
	public static let sample: Self = newBip39WordSample()
	public static let sampleOther: Self = newBip39WordSampleOther()
}
#endif // DEBUG
