import Foundation
import SargonUniFFI

#if DEBUG
extension FactorSources {
	public static let sample: Self = newFactorSourcesSample()
	public static let sampleOther: Self = newFactorSourcesSampleOther()
}
#endif // DEBUG
