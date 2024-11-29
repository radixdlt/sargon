import Foundation
import SargonUniFFI

#if DEBUG
extension [AssetException] {
	public static let sample: Self = newAssetsExceptionListSample()
	public static let sampleOther: Self = newAssetsExceptionListSampleOther()
}
#endif // DEBUG
