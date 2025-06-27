import Foundation
import SargonUniFFI

#if DEBUG
extension EmailAddress {
	public static let sample: Self = newEmailAddressSample()
	public static let sampleOther: Self = newEmailAddressSampleOther()
}
#endif // DEBUG
