import Foundation
import SargonUniFFI

#if DEBUG
extension AccountForDisplay {
	public static let sample: Self = newAccountForDisplaySample()
	public static let sampleOther: Self = newAccountForDisplaySampleOther()
}
#endif // DEBUG
