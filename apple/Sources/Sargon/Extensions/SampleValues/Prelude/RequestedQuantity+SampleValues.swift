import Foundation
import SargonUniFFI

#if DEBUG
extension RequestedQuantity {
	public static let sample: Self = newRequestedQuantitySample()
	public static let sampleOther: Self = newRequestedQuantitySampleOther()
}
#endif // DEBUG
