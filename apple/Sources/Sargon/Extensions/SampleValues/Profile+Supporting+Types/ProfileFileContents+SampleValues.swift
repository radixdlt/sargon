import Foundation
import SargonUniFFI

#if DEBUG
extension ProfileFileContents {
	public static let sample: Self = newProfileFileContentsSample()
	public static let sampleOther: Self = newProfileFileContentsSampleOther()
}
#endif // DEBUG
