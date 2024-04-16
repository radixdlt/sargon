import Foundation
import SargonUniFFI

#if DEBUG
extension Message {
	public static let sample: Self = newMessagePlaintextSample()
	public static let sampleOther: Self = newMessagePlaintextSampleOther()
}
#endif // DEBUG
