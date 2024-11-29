import Foundation
import SargonUniFFI

#if DEBUG
extension SharedPersonaData {
	public static let sample: Self = newSharedPersonaDataSample()
	public static let sampleOther: Self = newSharedPersonaDataSampleOther()
}
#endif // DEBUG
