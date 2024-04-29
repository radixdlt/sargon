import Foundation
import SargonUniFFI

#if DEBUG
extension AccountsOrPersonas {
	public static let sample: Self = newAccountsOrPersonasSample()
	public static let sampleOther: Self = newAccountsOrPersonasSampleOther()
}
#endif // DEBUG
