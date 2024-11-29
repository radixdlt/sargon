import Foundation
import SargonUniFFI

#if DEBUG
extension FactorSourceKind {
	public static let sample: Self = newFactorSourceKindSample()
	public static let sampleOther: Self = newFactorSourceKindSampleOther()
}
#endif // DEBUG
