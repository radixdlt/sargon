import Foundation
import SargonUniFFI

#if DEBUG
extension FactorSourceCryptoParameters {
	public static let sample: Self = newFactorSourceCryptoParametersSample()
	public static let sampleOther: Self = newFactorSourceCryptoParametersSampleOther()
}
#endif // DEBUG
