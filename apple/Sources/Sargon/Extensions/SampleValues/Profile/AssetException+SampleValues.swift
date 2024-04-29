import Foundation
import SargonUniFFI

#if DEBUG
extension AssetException {
	public static let sample: Self = newAssetExceptionSample()
	public static let sampleOther: Self = newAssetExceptionSampleOther()
}
#endif // DEBUG
