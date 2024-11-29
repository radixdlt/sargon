import Foundation
import SargonUniFFI

#if DEBUG
extension PasswordFactorSource {
	public static let sample: Self = newPasswordFactorSourceSample()

	public static let sampleOther: Self = newPasswordFactorSourceSampleOther()
}

#endif // DEBUG
