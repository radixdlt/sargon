import Foundation
import SargonUniFFI

#if DEBUG
extension HierarchicalDeterministicFactorInstance {
	public static let sample: Self = newHierarchicalDeterministicFactorInstanceSample()
	public static let sampleOther: Self = newHierarchicalDeterministicFactorInstanceSampleOther()
}
#endif // DEBUG
