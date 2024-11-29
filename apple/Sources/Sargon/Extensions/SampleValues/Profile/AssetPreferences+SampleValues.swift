import Foundation
import SargonUniFFI

#if DEBUG
extension [ResourceAppPreference] {
	public static let sample: Self = newResourcePreferencesSample()
	public static let sampleOther: Self = newResourcePreferencesSampleOther()
}
#endif // DEBUG
