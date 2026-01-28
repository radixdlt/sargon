import Foundation
import SargonUniFFI

// MARK: - SubintentManifest + CustomStringConvertible
extension SubintentManifest: CustomStringConvertible {
	public var description: String {
		manifestString
	}
}

extension SubintentManifest {
	public var manifestString: String {
		subintentManifestString(manifest: self)
	}

	public var blobs: Blobs {
		subintentManifestBlobs(manifest: self)
	}

	public var involvedPoolAddresses: [PoolAddress] {
		subintentManifestInvolvedPoolAddresses(manifest: self)
	}

	public var involvedResourceAddresses: [ResourceAddress] {
		subintentManifestInvolvedResourceAddresses(manifest: self)
	}
}
