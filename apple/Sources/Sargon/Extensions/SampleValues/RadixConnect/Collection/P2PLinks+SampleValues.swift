import Foundation
import SargonUniFFI

#if DEBUG
extension [P2PLink] {
	public static let sample: Self = newP2PLinksSample()
	public static let sampleOther: Self = newP2PLinksSampleOther()
}
#endif // DEBUG
