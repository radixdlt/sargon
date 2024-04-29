import Foundation
import SargonUniFFI

#if DEBUG
extension FiatCurrency {
	public static let sample: Self = newFiatCurrencySample()
	public static let sampleOther: Self = newFiatCurrencySampleOther()
}
#endif // DEBUG
