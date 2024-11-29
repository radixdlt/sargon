import Foundation
import SargonUniFFI

#if DEBUG
extension OffDeviceMnemonicFactorSource {
	public static let sample: Self = newOffDeviceMnemonicFactorSourceSample()

	public static let sampleOther: Self = newOffDeviceMnemonicFactorSourceSampleOther()
}

#endif // DEBUG
