import Foundation
import SargonUniFFI

#if DEBUG
extension [AuthorizedPersonaSimple] {
	public static let sample: Self = newReferencesToAuthorizedPersonasSample()
	public static let sampleOther: Self = newReferencesToAuthorizedPersonasSampleOther()
}
#endif // DEBUG
