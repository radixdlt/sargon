import Foundation
import SargonUniFFI

#if DEBUG
extension MnemonicWithPassphrase {
	public static let sample: Self = newMnemonicWithPassphraseSample()
	public static let sampleOther: Self = newMnemonicWithPassphraseSampleOther()
}
#endif // DEBUG
