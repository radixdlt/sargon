import Foundation
import SargonUniFFI

#if DEBUG
extension RadixConnectPurpose {
	public static let sample: Self = newRadixConnectPurposeSample()
	public static let sampleOther: Self = newRadixConnectPurposeSampleOther()
}
#endif // DEBUG