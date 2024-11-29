import Foundation
import SargonUniFFI

#if DEBUG
extension ArculusCardFactorSource {
	public static let sample: Self = newArculusCardFactorSourceSample()

	public static let sampleOther: Self = newArculusCardFactorSourceSampleOther()
}

#endif // DEBUG
