import Foundation
import SargonUniFFI

#if DEBUG
extension Header {
	public static let sample: Self = newHeaderSample()
	public static let sampleOther: Self = newHeaderSampleOther()
}
#endif // DEBUG
