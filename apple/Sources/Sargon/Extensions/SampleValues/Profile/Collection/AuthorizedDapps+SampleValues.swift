import Foundation
import SargonUniFFI

#if DEBUG
extension AuthorizedDapps {
	public static let sample: Self = newAuthorizedDappsSample()
	public static let sampleOther: Self = newAuthorizedDappsSampleOther()
}
#endif // DEBUG
