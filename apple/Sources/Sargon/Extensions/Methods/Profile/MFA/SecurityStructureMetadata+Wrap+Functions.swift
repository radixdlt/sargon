import Foundation
import SargonUniFFI

extension SecurityStructureMetadata {
	public init(name: DisplayName) {
		self = newSecurityStructureMetadataNamed(name: name)
	}

	public var isMain: Bool {
		securityStructureIsMain(securityStructureMetadata: self)
	}
}
