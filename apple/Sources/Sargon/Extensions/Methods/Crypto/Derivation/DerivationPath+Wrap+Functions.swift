import Foundation
import SargonUniFFI

extension DerivationPath {
	public var path: HDPath {
		derivationPathToHdPath(path: self)
	}

	public var canonicalBIP32: String {
		derivationPathToCanonicalBip32String(path: self)
	}

	public func toString() -> String {
		derivationPathToString(path: self)
	}

	public init(string: String) throws {
		self = try newDerivationPathFromString(string: string)
	}
}
