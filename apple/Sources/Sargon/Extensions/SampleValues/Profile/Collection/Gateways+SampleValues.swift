import Foundation
import SargonUniFFI

#if DEBUG
extension [Gateway] {
	public static let sample: Self = newGatewaysSample()
	public static let sampleOther: Self = newGatewaysSampleOther()
}
#endif // DEBUG
