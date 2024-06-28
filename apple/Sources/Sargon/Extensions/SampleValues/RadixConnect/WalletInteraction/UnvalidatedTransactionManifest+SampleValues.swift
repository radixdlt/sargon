import Foundation
import SargonUniFFI

#if DEBUG
extension UnvalidatedTransactionManifest {
	public static let sample: Self = newUnvalidatedTransactionManifestSample()
	public static let sampleOther: Self = newUnvalidatedTransactionManifestSampleOther()
}
#endif // DEBUG
