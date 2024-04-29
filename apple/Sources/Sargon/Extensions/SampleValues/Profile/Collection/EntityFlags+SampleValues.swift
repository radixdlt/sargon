import Foundation
import SargonUniFFI

#if DEBUG
extension EntityFlags {
	public static let sample: Self = newEntityFlagsSample()
	public static let sampleOther: Self = newEntityFlagsSampleOther()
}
#endif // DEBUG
