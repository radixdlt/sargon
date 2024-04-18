import Foundation
import SargonUniFFI

#if DEBUG
extension Gateway {
	public static let sample: Self = newGatewaySample()
	public static let sampleOther: Self = newGatewaySampleOther()
}
#endif
