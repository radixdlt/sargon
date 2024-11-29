import Foundation
import SargonUniFFI

#if DEBUG
extension AppPreferences {
	public static let sample: Self = newAppPreferencesSample()
	public static let sampleOther: Self = newAppPreferencesSampleOther()
}
#endif // DEBUG
