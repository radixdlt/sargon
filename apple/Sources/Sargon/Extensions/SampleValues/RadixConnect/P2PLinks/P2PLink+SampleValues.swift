import Foundation
import SargonUniFFI

#if DEBUG
extension P2PLink {
	public static let sample: Self = newP2pLinkSample()
	public static let sampleOther: Self = newP2pLinkSampleOther()
}
#endif // DEBUG
