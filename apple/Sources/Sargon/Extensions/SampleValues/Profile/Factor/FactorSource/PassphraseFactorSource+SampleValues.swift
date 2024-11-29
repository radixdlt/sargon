import Foundation
import SargonUniFFI

#if DEBUG
extension PassphraseFactorSource {
	public static let sample: Self = newPassphraseFactorSourceSample()

	public static let sampleOther: Self = newPassphraseFactorSourceSampleOther()
}

#endif // DEBUG
