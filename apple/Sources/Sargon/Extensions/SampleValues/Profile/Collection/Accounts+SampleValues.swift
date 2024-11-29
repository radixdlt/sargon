import Foundation
import SargonUniFFI

#if DEBUG
extension [Account] {
	public static let sample: Self = newAccountsSample()
	public static let sampleOther: Self = newAccountsSampleOther()
}
#endif // DEBUG
