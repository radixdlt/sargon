import Foundation
import SargonUniFFI

#if DEBUG
extension [Slip10Curve] {
	public static let sample: Self = newSupportedCurvesSample()
	public static let sampleOther: Self = newSupportedCurvesSampleOther()
}
#endif // DEBUG
