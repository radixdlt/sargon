import Foundation
import SargonUniFFI

#if DEBUG
extension [AccountOrPersona] {
	public static let sample: Self = newAccountsOrPersonasSample()
	public static let sampleOther: Self = newAccountsOrPersonasSampleOther()
}
#endif // DEBUG
