import Foundation
import SargonUniFFI

#if DEBUG
extension Gateway {
	public static let sample: Self = .mainnet
	public static let sampleOther: Self = .stokenet
}
#endif
