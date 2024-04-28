import Foundation
import SargonUniFFI

#if DEBUG
extension AccountOrPersona {
	public static let sampleMainnet: Self = newAccountOrPersonaSampleMainnet()
	public static let sampleMainnetOther: Self = newAccountOrPersonaSampleMainnetOther()
	public static let sampleMainnetThird: Self = newAccountOrPersonaSampleMainnetThird()

	public static let sampleStokenet: Self = newAccountOrPersonaSampleStokenet()
	public static let sampleStokenetOther: Self = newAccountOrPersonaSampleStokenetOther()
	public static let sampleStokenetThird: Self = newAccountOrPersonaSampleStokenetThird()
}
#endif
