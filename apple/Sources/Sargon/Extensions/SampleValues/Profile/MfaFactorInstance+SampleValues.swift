import Foundation
import SargonUniFFI

#if DEBUG
extension MfaFactorInstance {
	public static let sample: Self = newMfaFactorInstanceSample()
	public static let sampleOther: Self = newMfaFactorInstanceSampleOther()
}
#endif
