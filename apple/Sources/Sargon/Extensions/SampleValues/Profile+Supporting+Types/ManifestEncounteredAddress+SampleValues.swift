import Foundation
import SargonUniFFI

#if DEBUG
extension ManifestEncounteredAddress {
	public static let sampleMainnet: Self = newManifestEncounteredAddressSampleMainnet()
	public static let sampleMainnetOther: Self = newManifestEncounteredAddressSampleMainnetOther()

	public static let sampleStokenet: Self = newManifestEncounteredAddressSampleStokenet()
	public static let sampleStokenetOther: Self = newManifestEncounteredAddressSampleStokenetOther()
}
#endif
