import Foundation
import SargonUniFFI

#if DEBUG
extension [AuthorizedDapp] {
	public static let sample: Self = newAuthorizedDappsSample()
	public static let sampleOther: Self = newAuthorizedDappsSampleOther()
}
#endif // DEBUG
