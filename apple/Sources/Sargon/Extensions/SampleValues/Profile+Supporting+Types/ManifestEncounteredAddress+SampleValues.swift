import Foundation
import SargonUniFFI

#if DEBUG
extension ManifestEncounteredComponentAddress {
	public static let sampleMainnet: Self = newManifestEncounteredComponentAddressSampleMainnet()
	public static let sampleMainnetOther: Self = newManifestEncounteredComponentAddressSampleMainnetOther()

	public static let sampleStokenet: Self = newManifestEncounteredComponentAddressSampleStokenet()
	public static let sampleStokenetOther: Self = newManifestEncounteredComponentAddressSampleStokenetOther()
}
#endif
