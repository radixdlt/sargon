import Foundation
import SargonUniFFI

#if DEBUG
extension Personas {
	public static let sample: Self = newPersonasSample()
	public static let sampleOther: Self = newPersonasSampleOther()
}
#endif // DEBUG
